use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::cell::{Cell, CellPosition};
use crate::simulator::particle::Particle;
use crate::simulator::{NDIMS, PERIODICITIES};

use super::util;
use super::{Event, EventType, MoveToNeighbour};

pub fn schedule(
    lengths: &[f64; NDIMS],
    time: f64,
    cell: &Ref<Cell>,
    dim: usize,
    p: &Rc<RefCell<Particle>>,
) -> Option<Event> {
    let p_old: Ref<Particle> = p.borrow();
    let (dt, new_cell_index): (f64, usize) = {
        let length: f64 = lengths[dim];
        let cell_pos: &CellPosition = &cell.positions[dim];
        let periodicity: bool = PERIODICITIES[dim];
        let rad: f64 = p_old.rad;
        let pos: f64 = p_old.pos[dim];
        let vel: f64 = p_old.vel[dim];
        if 0. == vel {
            return None;
        }
        if vel < 0. {
            if !periodicity {
                match *cell_pos {
                    CellPosition::NegativeEdge => {
                        return None;
                    }
                    _ => {}
                }
            }
            let bound: f64 = cell.bounds[dim].min;
            let dpos: f64 = util::correct_periodicity(bound + rad - pos, length, cell_pos);
            let dt: f64 = dpos / vel;
            let neighbour: usize = cell.neighbours[dim].min;
            (dt, neighbour)
        } else {
            if !periodicity {
                match *cell_pos {
                    CellPosition::PositiveEdge => {
                        return None;
                    }
                    _ => {}
                }
            }
            let bound: f64 = cell.bounds[dim].max;
            let dpos: f64 = util::correct_periodicity(bound - rad - pos, length, cell_pos);
            let dt: f64 = dpos / vel;
            let neighbour: usize = cell.neighbours[dim].max;
            (dt, neighbour)
        }
    };
    if dt <= 0. {
        return None;
    }
    let event = MoveToNeighbour {
        cell_index: cell.index,
        p_old: p.clone(),
        p_new_pos: Particle::get_new_pos(lengths, p_old.pos, p_old.vel, dt),
        new_cell_index,
    };
    let event = Event {
        time: time + dt,
        eventtype: EventType::MoveToNeighbour(event),
    };
    Some(event)
}

pub fn execute(obj: &MoveToNeighbour, time: f64) -> &Rc<RefCell<Particle>> {
    let p: &Rc<RefCell<Particle>> = &obj.p_old;
    {
        let mut p: RefMut<Particle> = p.borrow_mut();
        p.pos = obj.p_new_pos;
        p.time = time;
    }
    p
}
