use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::cell::{Cell, CellPosition};
use crate::simulator::particle::Particle;
use crate::simulator::{NDIMS, PERIODICITIES};

use super::util;
use super::{Event, EventType, MoveOutOfCell};

pub fn schedule(
    lengths: &[f64; NDIMS],
    time: f64,
    cell: &Ref<Cell>,
    dim: usize,
    p: &Rc<RefCell<Particle>>,
) -> Option<Event> {
    let p_old: Ref<Particle> = p.borrow();
    let dt: f64 = {
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
            let dpos: f64 = util::correct_periodicity(bound - rad - pos, length, cell_pos);
            dpos / vel
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
            let dpos: f64 = util::correct_periodicity(bound + rad - pos, length, cell_pos);
            dpos / vel
        }
    };
    if dt <= 0. {
        return None;
    }
    let event = MoveOutOfCell {
        cell_index: cell.index,
        p_old: p.clone(),
        p_new_pos: Particle::get_new_pos(lengths, p_old.pos, p_old.vel, dt),
    };
    let event = Event {
        time: time + dt,
        eventtype: EventType::MoveOutOfCell(event),
    };
    Some(event)
}

pub fn execute(obj: &MoveOutOfCell, time: f64) -> &Rc<RefCell<Particle>> {
    let p: &Rc<RefCell<Particle>> = &obj.p_old;
    // TODO: maybe this update is not necessary,
    //   as the role of this event is to clean-up the relation
    //   with the old cell and the particle,
    //   and nothing new happens
    let mut p_mut: RefMut<Particle> = p.borrow_mut();
    p_mut.pos = obj.p_new_pos;
    p_mut.time = time;
    // anyway p should be returned though
    p
}
