use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::simulator::cell::{Cell, CellPosition};
use crate::simulator::extrema::Extrema;
use crate::simulator::particle::Particle;
use crate::simulator::Scheduler;
use crate::simulator::{Domain, NDIMS};

use super::{Event, EventType};

enum BoundaryCondition {
    Dirichlet(f64),
    Neumann,
}

pub struct WallReflection {
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    p_new_pos: MyVec,
    /// Velocity of the particle after this event.
    p_new_vel: MyVec,
    /// Scalar value after this event.
    p_new_val: f64,
}

impl WallReflection {
    pub fn schedule(
        domain: &Domain,
        time: f64,
        cell: &Rc<RefCell<Cell>>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        let periodicities: &[bool; NDIMS] = &domain.periodicities;
        // boundary conditions, only used when the direction is non-periodic
        let boundary_conditions: [Extrema<BoundaryCondition>; NDIMS] = [
            Extrema::<BoundaryCondition> {
                min: BoundaryCondition::Neumann,
                max: BoundaryCondition::Neumann,
            },
            Extrema::<BoundaryCondition> {
                min: BoundaryCondition::Dirichlet(0.),
                max: BoundaryCondition::Dirichlet(1.),
            },
        ];
        let p_old: Ref<Particle> = p.borrow();
        let (dt, p_new_vel, p_new_val): (f64, MyVec, f64) = {
            // schedule only if the direction is wall-bounded (non-periodic)
            let periodicity: bool = periodicities[dim];
            if periodicity {
                return None;
            }
            let cell_pos: &CellPosition = &cell.borrow().positions[dim];
            let rad: f64 = p_old.rad;
            let pos: f64 = p_old.pos[dim];
            let vel: f64 = p_old.vel[dim];
            if 0. == vel {
                return None;
            }
            if vel < 0. {
                match *cell_pos {
                    CellPosition::NegativeEdge => {}
                    _ => return None,
                }
                let bound: f64 = cell.borrow().bounds[dim].min;
                let dpos: f64 = bound + rad - pos;
                let mut p_new_vel: MyVec = p_old.vel;
                p_new_vel[dim] = -1. * vel;
                let p_new_val: f64 = match boundary_conditions[dim].min {
                    BoundaryCondition::Dirichlet(val) => 0.5 * (val + p_old.val),
                    BoundaryCondition::Neumann => p_old.val,
                };
                (dpos / vel, p_new_vel, p_new_val)
            } else {
                match *cell_pos {
                    CellPosition::PositiveEdge => {}
                    _ => return None,
                }
                let bound: f64 = cell.borrow().bounds[dim].max;
                let dpos: f64 = bound - rad - pos;
                let mut p_new_vel: MyVec = p_old.vel;
                p_new_vel[dim] = -1. * vel;
                let p_new_val: f64 = match boundary_conditions[dim].max {
                    BoundaryCondition::Dirichlet(val) => 0.5 * (val + p_old.val),
                    BoundaryCondition::Neumann => p_old.val,
                };
                (dpos / vel, p_new_vel, p_new_val)
            }
        };
        if dt <= 0. {
            return None;
        }
        let event = WallReflection {
            p_old: p.clone(),
            p_new_pos: Particle::get_new_pos(domain, p_old.pos, p_old.vel, dt),
            p_new_vel,
            p_new_val,
        };
        let event = Event {
            time: time + dt,
            eventtype: EventType::WallReflection(event),
        };
        Some(event)
    }

    pub fn execute(&self, domain: &Domain, time: f64, scheduler: &mut Scheduler) {
        let p: &Rc<RefCell<Particle>> = &self.p_old;
        {
            let mut p: RefMut<Particle> = p.borrow_mut();
            p.pos = self.p_new_pos;
            p.vel = self.p_new_vel;
            p.val = self.p_new_val;
            p.time = time;
        }
        // cancel all events related to these two particles
        //   since their velocities are altered
        for cell in p.borrow().cells.iter() {
            super::cancel_events(p, cell, scheduler);
        }
        // reschedule all events related to these two particles
        for cell in p.borrow().cells.iter() {
            super::schedule_events(domain, p, cell, scheduler);
        }
    }
}
