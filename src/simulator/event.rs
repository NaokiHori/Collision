mod inter_particle_collision;
mod move_out_of_cell;
mod move_to_neighbour;
mod synchronisation;
mod util;
mod wall_reflection;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::cell::Cell;
use crate::simulator::particle::Particle;
use crate::simulator::scheduler::Scheduler;
use crate::simulator::NDIMS;

use inter_particle_collision::InterParticleCollision;
use move_out_of_cell::MoveOutOfCell;
use move_to_neighbour::MoveToNeighbour;
use synchronisation::Synchronisation;
use wall_reflection::WallReflection;

pub enum EventType {
    InterParticleCollision(InterParticleCollision),
    MoveToNeighbour(MoveToNeighbour),
    MoveOutOfCell(MoveOutOfCell),
    WallReflection(WallReflection),
    Synchronisation(Synchronisation),
}

pub struct Event {
    pub time: f64,
    pub eventtype: EventType,
}

fn get_head_event_time(events: &Rc<RefCell<Vec<Event>>>) -> f64 {
    let events: Ref<Vec<Event>> = events.borrow();
    if events.is_empty() {
        f64::MAX
    } else {
        events[0].time
    }
}

/// Inserts a new event to the given cell in a sorted order.
///
/// # Arguments
/// * `new_event` - a new event to be appended.
/// * `cell`      - the cell which is of interest.
/// * `scheduler` - a minimum heap to find the latest event.
fn insert_event(new_event: Event, cell: &Rc<RefCell<Cell>>, scheduler: &mut Scheduler) {
    let cell: Ref<Cell> = cell.borrow();
    // store head event before inisertion
    let time_bef: f64 = get_head_event_time(&cell.events);
    // insert
    {
        let mut events: RefMut<Vec<Event>> = cell.events.borrow_mut();
        match events.binary_search_by(|event| event.time.partial_cmp(&new_event.time).unwrap()) {
            Ok(position) | Err(position) => events.insert(position, new_event),
        };
    }
    // store head event after inisertion
    let time_aft: f64 = get_head_event_time(&cell.events);
    // update heap
    scheduler.update(cell.index, time_bef, time_aft);
}

/// For each cell, checks events and inserts them if applicable.
pub fn init_events(lengths: &[f64; NDIMS], cells: &[Rc<RefCell<Cell>>], scheduler: &mut Scheduler) {
    let time: f64 = 0.;
    for cell in cells.iter() {
        let cell_borrowed: Ref<Cell> = cell.borrow();
        // main cell handles the synchronisation
        if 0 == cell_borrowed.index {
            insert_event(Synchronisation::schedule(time, cell), cell, scheduler);
        }
        // append inter-particle events
        let particles: Ref<Vec<Rc<RefCell<Particle>>>> = cell_borrowed.particles.borrow();
        for (n, p) in particles.iter().enumerate() {
            for q in particles[n + 1..].iter() {
                if let Some(event) = InterParticleCollision::schedule(lengths, time, cell, p, q) {
                    insert_event(event, cell, scheduler);
                }
            }
        }
        // append boundary events
        for p in particles.iter() {
            for dim in 0..NDIMS {
                if let Some(event) = MoveToNeighbour::schedule(lengths, time, cell, dim, p) {
                    insert_event(event, cell, scheduler);
                }
            }
            for dim in 0..NDIMS {
                if let Some(event) = MoveOutOfCell::schedule(lengths, time, cell, dim, p) {
                    insert_event(event, cell, scheduler);
                }
            }
            for dim in 0..NDIMS {
                if let Some(event) = WallReflection::schedule(lengths, time, cell, dim, p) {
                    insert_event(event, cell, scheduler);
                }
            }
        }
    }
}

/// Checks and inserts new events related to "p" into the series of events
fn schedule_events(
    lengths: &[f64; NDIMS],
    p: &Rc<RefCell<Particle>>,
    cell: &Rc<RefCell<Cell>>,
    scheduler: &mut Scheduler,
) {
    if cfg!(debug_assertions) {
        crate::simulator::debug::check_recognition(p, cell);
    }
    let cell_borrowed: Ref<Cell> = cell.borrow();
    // get local time of the given particle
    let time: f64 = p.borrow().time;
    // all particles in this cell
    let qs: Ref<Vec<Rc<RefCell<Particle>>>> = cell_borrowed.particles.borrow();
    // update all particles in the cell to the local time of the given particle,
    //   so that the interactions can be correctly handled
    for q in qs.iter() {
        if Rc::ptr_eq(p, q) {
            continue;
        }
        let mut q: RefMut<Particle> = q.borrow_mut();
        q.pos = Particle::get_new_pos(lengths, q.pos, q.vel, time - q.time);
        q.time = time;
    }
    for q in qs.iter() {
        if Rc::ptr_eq(p, q) {
            continue;
        }
        if let Some(event) = InterParticleCollision::schedule(lengths, time, cell, p, q) {
            insert_event(event, cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = MoveToNeighbour::schedule(lengths, time, cell, dim, p) {
            insert_event(event, cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = MoveOutOfCell::schedule(lengths, time, cell, dim, p) {
            insert_event(event, cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = WallReflection::schedule(lengths, time, cell, dim, p) {
            insert_event(event, cell, scheduler);
        }
    }
}

/// Cancels all events which involve the specified particle.
///
/// # Arguments
/// * `p`         - a particle whose events are to be removed.
/// * `cell`      - a cell whose events which involve the given particle are to be removed.
/// * `scheduler` - a minimum heap to find the next event.
fn cancel_events(p: &Rc<RefCell<Particle>>, cell: &Rc<RefCell<Cell>>, scheduler: &mut Scheduler) {
    if cfg!(debug_assertions) {
        crate::simulator::debug::check_recognition(p, cell);
    }
    let cell: Ref<Cell> = cell.borrow();
    // get the head event before the event list is modified
    let time_bef: f64 = get_head_event_time(&cell.events);
    // only keep events which do not involve p
    cell.events
        .borrow_mut()
        .retain(|event| match &event.eventtype {
            EventType::InterParticleCollision(event) => {
                !Rc::ptr_eq(p, &event.p_old) && !Rc::ptr_eq(p, &event.q_old)
            }
            EventType::MoveToNeighbour(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::MoveOutOfCell(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::WallReflection(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::Synchronisation(_) => true,
        });
    // get the head event after the event list is modified
    let time_aft: f64 = get_head_event_time(&cell.events);
    // update heap
    // NOTE: although this process is not necessary when the head event is not altered,
    //   I perform this anyway since the overhead is small,
    //   i.e. it attempts a down-shift, which is turned out to be not necessary,
    //     then immediately quit
    scheduler.update(cell.index, time_bef, time_aft);
}

/// Core function.
///
/// This function processes events until a synchronisation (all particles are at the same time).
/// For each event, the whole process is as follows:
/// 1. Picks up the next event
/// 2. Updates involved particles, e.g. updating velocity
/// 3. Cancels out-dated events and reschedule new events
pub fn process_events(
    lengths: &[f64; NDIMS],
    particles: &[Rc<RefCell<Particle>>],
    cells: &[Rc<RefCell<Cell>>],
    scheduler: &mut Scheduler,
    sync_rate: f64,
) -> f64 {
    // loop until the desired time (synchronised)
    let time: f64 = loop {
        // take out the next event from the minumum heap
        let event: Event = {
            // get the cell in which the next event happens
            let cell: Rc<RefCell<Cell>> = scheduler.get();
            let cell: Ref<Cell> = cell.borrow();
            // trim the first element of the event list
            let event_bef: Event = cell.events.borrow_mut().remove(0);
            let time_bef: f64 = event_bef.time;
            // now the latest event is extracted and the heap is altered as well
            // I need to fix it so that it is balanced again
            let time_aft: f64 = get_head_event_time(&cell.events);
            scheduler.update(cell.index, time_bef, time_aft);
            // the event to be processed is this
            event_bef
        };
        // process the extracted event
        let time: f64 = event.time;
        match &event.eventtype {
            EventType::InterParticleCollision(event) => {
                // inter-particle collision
                // update particle positions / velocities,
                //   cancel all involved events,
                //   reschedule events in all involved cells
                event.execute(lengths, time, scheduler);
            }
            EventType::MoveToNeighbour(event) => {
                // one particle is almost getting out of the cell
                // I need to tell the information of it to the neighbouring cell
                //   which is present in the direction of the particle motion
                event.execute(lengths, time, scheduler, cells);
            }
            EventType::MoveOutOfCell(event) => {
                // one particle has left the cell
                // the cell forgets the particle,
                //   and the particle forgets the cell
                event.execute(time, scheduler);
            }
            EventType::WallReflection(event) => {
                // update particle reflecting on the wall
                // NOTE: only when the direction is not periodic
                event.execute(lengths, time, scheduler);
            }
            EventType::Synchronisation(event) => {
                // update all particles to the desired time to synchronise for output
                // after this event exit the loop to draw state
                event.execute(lengths, time, sync_rate, particles, scheduler);
                break time;
            }
        }
    };
    time
}
