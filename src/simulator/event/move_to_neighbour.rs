use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::simulator::cell::{Cell, CellPosition};
use crate::simulator::particle::Particle;
use crate::simulator::scheduler::Scheduler;
use crate::simulator::{Domain, NDIMS};

use super::util;
use super::{Event, EventType};

pub struct MoveToNeighbour {
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    p_new_pos: MyVec,
    /// Index of the cell, to which the particle information is passed.
    new_cell_index: usize,
}

impl MoveToNeighbour {
    pub fn schedule(
        domain: &Domain,
        time: f64,
        cell: &Rc<RefCell<Cell>>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        let lengths: &[f64; NDIMS] = &domain.lengths;
        let periodicities: &[bool; NDIMS] = &domain.periodicities;
        let p_old: Ref<Particle> = p.borrow();
        let (dt, new_cell_index): (f64, usize) = {
            let length: f64 = lengths[dim];
            let cell_pos: &CellPosition = &cell.borrow().positions[dim];
            let periodicity: bool = periodicities[dim];
            let rad: f64 = p_old.rad;
            let pos: f64 = p_old.pos[dim];
            let vel: f64 = p_old.vel[dim];
            if 0. == vel {
                return None;
            }
            if vel < 0. {
                if !periodicity {
                    if let CellPosition::NegativeEdge = *cell_pos {
                        return None;
                    }
                }
                let bound: f64 = cell.borrow().bounds[dim].min;
                let dpos: f64 = util::correct_periodicity(bound + rad - pos, length, cell_pos);
                let dt: f64 = dpos / vel;
                let neighbour: usize = cell.borrow().neighbours[dim].min;
                (dt, neighbour)
            } else {
                if !periodicity {
                    if let CellPosition::PositiveEdge = *cell_pos {
                        return None;
                    }
                }
                let bound: f64 = cell.borrow().bounds[dim].max;
                let dpos: f64 = util::correct_periodicity(bound - rad - pos, length, cell_pos);
                let dt: f64 = dpos / vel;
                let neighbour: usize = cell.borrow().neighbours[dim].max;
                (dt, neighbour)
            }
        };
        if dt <= 0. {
            return None;
        }
        let event = MoveToNeighbour {
            p_old: p.clone(),
            p_new_pos: Particle::get_new_pos(domain, p_old.pos, p_old.vel, dt),
            new_cell_index,
        };
        let event = Event {
            time: time + dt,
            eventtype: EventType::MoveToNeighbour(event),
        };
        Some(event)
    }

    pub fn execute(
        &self,
        domain: &Domain,
        time: f64,
        scheduler: &mut Scheduler,
        cells: &[Rc<RefCell<Cell>>],
    ) {
        let p: &Rc<RefCell<Particle>> = &self.p_old;
        {
            let mut p: RefMut<Particle> = p.borrow_mut();
            p.pos = self.p_new_pos;
            p.time = time;
        }
        // for the new cell,
        //   1. register this particle to the next cell
        //   2. register the cell index to the list
        //   3. schedule events of the particle in the new cell
        let cell_index: usize = self.new_cell_index;
        let cell: &Rc<RefCell<Cell>> = &cells[cell_index];
        cell.borrow_mut().append(p);
        p.borrow_mut().append(cell);
        super::schedule_events(domain, p, cell, scheduler);
    }
}
