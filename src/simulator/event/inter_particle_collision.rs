use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::simulator::cell::Cell;
use crate::simulator::particle::Particle;
use crate::simulator::NDIMS;

use super::util;
use super::{Event, EventType, InterParticleCollision};

/// Pseudo restitution coefficient.
const RESTCOEF: f64 = 0.99;

pub fn schedule(
    lengths: &[f64; NDIMS],
    time: f64,
    cell: &Ref<Cell>,
    p: &Rc<RefCell<Particle>>,
    q: &Rc<RefCell<Particle>>,
) -> Option<Event> {
    let gravity = MyVec::new([0., -0.5]);
    let p_old: Ref<Particle> = p.borrow();
    let q_old: Ref<Particle> = q.borrow();
    // x0' = x0 + v0 dt
    // x1' = x1 + v1 dt
    // I want to know dt when |x1' - x0'| = r0 + r1, leading to
    // dv^2 dt^2 + 2 dv dx dt + dx^2 - (r0 + r1)^2 = 0
    // or
    // a dt^2 + 2 b dt + c = 0
    // or
    // dt = 1 / a * ( - b [+-] sqrt(b^2 - a c) )
    let mut dpos: MyVec = q_old.pos - p_old.pos;
    let dvel: MyVec = q_old.vel - p_old.vel;
    for dim in 0..NDIMS {
        dpos[dim] = util::correct_periodicity(dpos[dim], lengths[dim], &cell.positions[dim]);
    }
    let a: f64 = dvel * dvel;
    let b: f64 = dvel * dpos;
    let c: f64 = dpos * dpos - (p_old.rad + q_old.rad).powi(2);
    let d: f64 = b.powi(2) - a * c;
    // a is non-negative
    // exclude 0
    if a < f64::EPSILON {
        return None;
    }
    // when the displacement and the velocity vectors direct to the same direction,
    //   the two particles never collide in the future
    if 0. <= b {
        return None;
    }
    // two particles are (slightly) overlapped,
    //   which may happen because of the rounding errors ust after collisions
    if c < 0. {
        return None;
    }
    // discreminant, no solution when negative
    if d < 0. {
        return None;
    }
    // there are two solutions:
    //   (-b + sqrt(d)) / a
    //   (-b - sqrt(d)) / a
    // I am interested in the smaller (earlier) solution, which is the latter
    //   provided a > 0 and b < 0
    let dt: f64 = 1. / a * (-d.sqrt() - b);
    if dt < 0. {
        return None;
    }
    // get positions and velocities after collision
    // consider a coordinate system moving with the center of mass,
    //   interchange the velocity
    let (p_new_pos, q_new_pos, p_new_vel, q_new_vel, p_new_val, q_new_val): (
        MyVec,
        MyVec,
        MyVec,
        MyVec,
        f64,
        f64,
    ) = {
        let p_new_pos: MyVec = Particle::get_new_pos(lengths, p_old.pos, p_old.vel, dt);
        let q_new_pos: MyVec = Particle::get_new_pos(lengths, q_old.pos, q_old.vel, dt);
        let new_val: f64 = 0.5 * p_old.val + 0.5 * q_old.val;
        let p_new_val: f64 = new_val;
        let q_new_val: f64 = new_val;
        // displacement with the periodicity considered
        let dpos: MyVec = {
            let mut dpos: MyVec = q_new_pos - p_new_pos;
            for dim in 0..NDIMS {
                dpos[dim] =
                    util::correct_periodicity(dpos[dim], lengths[dim], &cell.positions[dim]);
            }
            dpos
        };
        // normal vector connecting particle centres
        let normal: MyVec = dpos / (p_old.rad + q_old.rad);
        // gravity-centre velocity
        // NOTE: pseudo gravity is added
        let gvel: MyVec = 0.5 * p_old.vel + 0.5 * q_old.vel + (new_val - 0.5) * gravity;
        // velocity difference after collision in the centre-of-mass coordinate
        let dvel = dvel - (1. + RESTCOEF) * (dvel * normal) * normal;
        // go back to the original coordinate
        (
            p_new_pos,
            q_new_pos,
            gvel - 0.5 * dvel,
            gvel + 0.5 * dvel,
            p_new_val,
            q_new_val,
        )
    };
    let event = InterParticleCollision {
        cell_index: cell.index,
        p_old: p.clone(),
        q_old: q.clone(),
        p_new_pos,
        q_new_pos,
        p_new_vel,
        q_new_vel,
        p_new_val,
        q_new_val,
    };
    let event = Event {
        time: time + dt,
        eventtype: EventType::InterParticleCollision(event),
    };
    Some(event)
}

pub fn execute(
    obj: &InterParticleCollision,
    time: f64,
) -> (&Rc<RefCell<Particle>>, &Rc<RefCell<Particle>>) {
    let p: &Rc<RefCell<Particle>> = &obj.p_old;
    let q: &Rc<RefCell<Particle>> = &obj.q_old;
    let mut p_mut: RefMut<Particle> = p.borrow_mut();
    let mut q_mut: RefMut<Particle> = q.borrow_mut();
    p_mut.pos = obj.p_new_pos;
    q_mut.pos = obj.q_new_pos;
    p_mut.vel = obj.p_new_vel;
    q_mut.vel = obj.q_new_vel;
    p_mut.val = obj.p_new_val;
    q_mut.val = obj.q_new_val;
    p_mut.time = time;
    q_mut.time = time;
    (p, q)
}
