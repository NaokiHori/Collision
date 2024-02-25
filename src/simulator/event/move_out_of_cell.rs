use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::simulator::cell::{Cell, CellPosition};
use crate::simulator::particle::Particle;
use crate::simulator::scheduler::Scheduler;
use crate::simulator::{NDIMS, PERIODICITIES};

use super::util;
use super::{Event, EventType};

pub struct MoveOutOfCell {
    /// The cell in which this event happens.
    cell: Rc<RefCell<Cell>>,
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    p_new_pos: MyVec,
}

impl MoveOutOfCell {
    pub fn schedule(
        lengths: &[f64; NDIMS],
        time: f64,
        cell: &Rc<RefCell<Cell>>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        let p_old: Ref<Particle> = p.borrow();
        let dt: f64 = {
            let length: f64 = lengths[dim];
            let cell_pos: &CellPosition = &cell.borrow().positions[dim];
            let periodicity: bool = PERIODICITIES[dim];
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
                let dpos: f64 = util::correct_periodicity(bound - rad - pos, length, cell_pos);
                dpos / vel
            } else {
                if !periodicity {
                    if let CellPosition::PositiveEdge = *cell_pos {
                        return None;
                    }
                }
                let bound: f64 = cell.borrow().bounds[dim].max;
                let dpos: f64 = util::correct_periodicity(bound + rad - pos, length, cell_pos);
                dpos / vel
            }
        };
        if dt <= 0. {
            return None;
        }
        let event = MoveOutOfCell {
            cell: cell.clone(),
            p_old: p.clone(),
            p_new_pos: Particle::get_new_pos(lengths, p_old.pos, p_old.vel, dt),
        };
        let event = Event {
            time: time + dt,
            eventtype: EventType::MoveOutOfCell(event),
        };
        Some(event)
    }

    pub fn execute(&self, time: f64, scheduler: &mut Scheduler) {
        let p: &Rc<RefCell<Particle>> = &self.p_old;
        {
            let mut p_mut: RefMut<Particle> = p.borrow_mut();
            p_mut.pos = self.p_new_pos;
            p_mut.time = time;
        }
        // for the cell from which the particle is leaving,
        //   1. cancel events related to this particle in the old cell
        //   2. remove the particle from the local particle list
        //   3. remove the cell from the cell list
        let cell: &Rc<RefCell<Cell>> = &self.cell;
        super::cancel_events(p, cell, scheduler);
        cell.borrow_mut().remove(p);
        p.borrow_mut().remove(cell);
    }
}
