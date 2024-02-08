mod inter_particle_collision;
mod move_out_of_cell;
mod move_to_neighbour;
mod synchronisation;
mod util;
mod wall_reflection;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::simulator::cell::Cell;
use crate::simulator::particle::Particle;
use crate::simulator::scheduler::Scheduler;
use crate::simulator::NDIMS;

pub struct InterParticleCollision {
    pub cell_index: usize,
    pub p_old: Rc<RefCell<Particle>>,
    pub q_old: Rc<RefCell<Particle>>,
    pub p_new_pos: MyVec,
    pub q_new_pos: MyVec,
    pub p_new_vel: MyVec,
    pub q_new_vel: MyVec,
    pub p_new_val: f64,
    pub q_new_val: f64,
}

impl InterParticleCollision {
    pub fn schedule(
        lengths: &[f64; NDIMS],
        time: f64,
        cell: &Ref<Cell>,
        p: &Rc<RefCell<Particle>>,
        q: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        inter_particle_collision::schedule(lengths, time, cell, p, q)
    }

    pub fn execute(&self, time: f64) -> (&Rc<RefCell<Particle>>, &Rc<RefCell<Particle>>) {
        inter_particle_collision::execute(self, time)
    }
}

pub struct MoveToNeighbour {
    /// Index of the cell, in which this event happens.
    pub cell_index: usize,
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    pub p_new_pos: MyVec,
    /// Index of the cell, to which the particle information is passed.
    pub new_cell_index: usize,
}

impl MoveToNeighbour {
    pub fn schedule(
        lengths: &[f64; NDIMS],
        time: f64,
        cell: &Ref<Cell>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        move_to_neighbour::schedule(lengths, time, cell, dim, p)
    }

    pub fn execute(&self, time: f64) -> &Rc<RefCell<Particle>> {
        move_to_neighbour::execute(self, time)
    }
}

pub struct MoveOutOfCell {
    /// Index of the cell, in which this event happens.
    pub cell_index: usize,
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    pub p_new_pos: MyVec,
}

impl MoveOutOfCell {
    pub fn schedule(
        lengths: &[f64; NDIMS],
        time: f64,
        cell: &Ref<Cell>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        move_out_of_cell::schedule(lengths, time, cell, dim, p)
    }

    pub fn execute(&self, time: f64) -> &Rc<RefCell<Particle>> {
        move_out_of_cell::execute(self, time)
    }
}

pub struct WallReflection {
    /// Index of the cell, in which this event happens.
    pub cell_index: usize,
    /// Reference to the involved particle.
    pub p_old: Rc<RefCell<Particle>>,
    /// Position of the particle after this event.
    pub p_new_pos: MyVec,
    /// Velocity of the particle after this event.
    pub p_new_vel: MyVec,
    /// Scalar value after this event.
    pub p_new_val: f64,
}

impl WallReflection {
    pub fn schedule(
        lengths: &[f64; NDIMS],
        time: f64,
        cell: &Ref<Cell>,
        dim: usize,
        p: &Rc<RefCell<Particle>>,
    ) -> Option<Event> {
        wall_reflection::schedule(lengths, time, cell, dim, p)
    }

    pub fn execute(&self, time: f64) -> &Rc<RefCell<Particle>> {
        wall_reflection::execute(self, time)
    }
}

pub struct Synchronisation {}

impl Synchronisation {
    pub fn schedule(time: f64) -> Event {
        synchronisation::schedule(time)
    }

    pub fn execute(&self, lengths: &[f64; NDIMS], time: f64, particles: &[Rc<RefCell<Particle>>]) {
        synchronisation::execute(lengths, time, particles)
    }
}

pub enum EventType {
    InterParticleCollision(InterParticleCollision),
    MoveToNeighbour(MoveToNeighbour),
    MoveOutOfCell(MoveOutOfCell),
    WallReflection(WallReflection),
    Synchronisation(Synchronisation),
    NotApplicable,
}

pub struct Event {
    pub time: f64,
    pub eventtype: EventType,
}

fn get_head_event(events: &Rc<RefCell<Vec<Rc<RefCell<Event>>>>>) -> Rc<RefCell<Event>> {
    let events: Ref<Vec<Rc<RefCell<Event>>>> = events.borrow();
    if events.is_empty() {
        Rc::new(RefCell::new(Event {
            time: f64::MAX,
            eventtype: EventType::NotApplicable,
        }))
    } else {
        events[0].clone()
    }
}

/// Inserts a new event to the given cell in a sorted order.
///
/// # Arguments
/// * `new_event` - a new event to be appended.
/// * `cell`      - the cell which is of interest.
/// * `scheduler` - a minimum heap to find the latest event.
fn insert_event(new_event: Event, cell: &Ref<Cell>, scheduler: &mut Scheduler) {
    // store head event before inisertion
    let event_bef: Rc<RefCell<Event>> = get_head_event(&cell.events);
    // insert
    {
        let mut events: RefMut<Vec<Rc<RefCell<Event>>>> = cell.events.borrow_mut();
        match events
            .binary_search_by(|event| event.borrow().time.partial_cmp(&new_event.time).unwrap())
        {
            Ok(position) | Err(position) => {
                events.insert(position, Rc::new(RefCell::new(new_event)));
            }
        };
    }
    // store head event after inisertion
    let event_aft: Rc<RefCell<Event>> = get_head_event(&cell.events);
    // update heap
    scheduler.update(cell.index, &event_bef, &event_aft);
}

/// For each cell, checks events and inserts them if applicable.
pub fn init_events(lengths: &[f64; NDIMS], cells: &[Rc<RefCell<Cell>>], scheduler: &mut Scheduler) {
    let time: f64 = 0.;
    for cell in cells.iter() {
        let cell: Ref<Cell> = cell.borrow();
        // main cell handles the synchronisation
        if 0 == cell.index {
            insert_event(Synchronisation::schedule(time), &cell, scheduler);
        }
        // append inter-particle events
        let particles: Ref<Vec<Rc<RefCell<Particle>>>> = cell.particles.borrow();
        for (n, p) in particles.iter().enumerate() {
            for q in particles[n + 1..].iter() {
                if let Some(event) = InterParticleCollision::schedule(lengths, time, &cell, p, q) {
                    insert_event(event, &cell, scheduler);
                }
            }
        }
        // append boundary events
        for p in particles.iter() {
            for dim in 0..NDIMS {
                if let Some(event) = MoveToNeighbour::schedule(lengths, time, &cell, dim, p) {
                    insert_event(event, &cell, scheduler);
                }
            }
            for dim in 0..NDIMS {
                if let Some(event) = MoveOutOfCell::schedule(lengths, time, &cell, dim, p) {
                    insert_event(event, &cell, scheduler);
                }
            }
            for dim in 0..NDIMS {
                if let Some(event) = WallReflection::schedule(lengths, time, &cell, dim, p) {
                    insert_event(event, &cell, scheduler);
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
        crate::simulator::debug::check_recognition(p, &cell.borrow());
    }
    // get local time of the given particle
    let time: f64 = p.borrow().time;
    // all particles in this cell
    let cell: Ref<Cell> = cell.borrow();
    let qs: Ref<Vec<Rc<RefCell<Particle>>>> = cell.particles.borrow();
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
        if let Some(event) = InterParticleCollision::schedule(lengths, time, &cell, p, q) {
            insert_event(event, &cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = MoveToNeighbour::schedule(lengths, time, &cell, dim, p) {
            insert_event(event, &cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = MoveOutOfCell::schedule(lengths, time, &cell, dim, p) {
            insert_event(event, &cell, scheduler);
        }
    }
    for dim in 0..NDIMS {
        if let Some(event) = WallReflection::schedule(lengths, time, &cell, dim, p) {
            insert_event(event, &cell, scheduler);
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
    let cell: Ref<Cell> = cell.borrow();
    if cfg!(debug_assertions) {
        crate::simulator::debug::check_recognition(p, &cell);
    }
    // get the head event before the event list is modified
    let event_bef: Rc<RefCell<Event>> = get_head_event(&cell.events);
    // only keep events which do not involve p
    cell.events.borrow_mut().retain(|event| {
        let event: Ref<Event> = event.borrow();
        match &event.eventtype {
            EventType::InterParticleCollision(event) => {
                !Rc::ptr_eq(p, &event.p_old) && !Rc::ptr_eq(p, &event.q_old)
            }
            EventType::MoveToNeighbour(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::MoveOutOfCell(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::WallReflection(event) => !Rc::ptr_eq(p, &event.p_old),
            EventType::Synchronisation(_) => true,
            EventType::NotApplicable => true,
        }
    });
    // get the head event after the event list is modified
    let event_aft: Rc<RefCell<Event>> = get_head_event(&cell.events);
    // update heap
    // NOTE: although this process is not necessary when the head event is not altered,
    //   I perform this anyway since the overhead is small,
    //   i.e. it attempts a down-shift, which is turned out to be not necessary,
    //     then immediately quit
    scheduler.update(cell.index, &event_bef, &event_aft);
}

/// Core function.
///
/// The whole process is as follows:
/// 1. Picks up the latest event
/// 2. Treats objects which are involved in the event, e.g. updating velocity
/// 3. Cancels out-dated events and reschedule new events
pub fn process_events(
    lengths: &[f64; NDIMS],
    particles: &[Rc<RefCell<Particle>>],
    cells: &[Rc<RefCell<Cell>>],
    scheduler: &mut Scheduler,
    sync_rate: f64,
) -> f64 {
    // loop until the desired time
    let time: f64 = loop {
        // take out the coming event from the minumum heap
        let event: Rc<RefCell<Event>> = {
            // get the cell in which the next event happens
            let cell: Rc<RefCell<Cell>> = scheduler.get();
            let cell: Ref<Cell> = cell.borrow();
            // trim the first element of the event list: next event
            let event_bef: Rc<RefCell<Event>> = cell.events.borrow_mut().remove(0);
            // now the latest event is extracted and the heap is altered as well
            // I need to fix it so that it is balanced again
            let event_aft: Rc<RefCell<Event>> = get_head_event(&cell.events);
            scheduler.update(cell.index, &event_bef, &event_aft);
            event_bef
        };
        // process the extracted event
        let event: Ref<Event> = event.borrow();
        let time: f64 = event.time;
        match &event.eventtype {
            // inter-particle collision occurs
            EventType::InterParticleCollision(event) => {
                // update particles
                let (p, q): (&Rc<RefCell<Particle>>, &Rc<RefCell<Particle>>) = event.execute(time);
                // cancel all events related to these two particles
                //   since their velocities are altered
                for &cell_index in p.borrow().cell_indices.iter() {
                    cancel_events(p, &cells[cell_index], scheduler);
                }
                for &cell_index in q.borrow().cell_indices.iter() {
                    cancel_events(q, &cells[cell_index], scheduler);
                }
                // reschedule all events related to these two particles
                for &cell_index in p.borrow().cell_indices.iter() {
                    schedule_events(lengths, p, &cells[cell_index], scheduler);
                }
                for &cell_index in q.borrow().cell_indices.iter() {
                    schedule_events(lengths, q, &cells[cell_index], scheduler);
                }
            }
            // one particle is almost getting out of the cell
            // I need to tell the information of it to the neighbouring cell
            //   which is present in the direction of the particle motion
            EventType::MoveToNeighbour(event) => {
                // update particles
                let p: &Rc<RefCell<Particle>> = event.execute(time);
                // for the new cell,
                //   1. register this particle to the next cell
                //   2. register the cell index to the list
                //   3. schedule events of the particle in the new cell
                let cell_index: usize = event.new_cell_index;
                let cell: &Rc<RefCell<Cell>> = &cells[cell_index];
                cell.borrow_mut().append(p);
                p.borrow_mut().append(cell_index);
                schedule_events(lengths, p, cell, scheduler);
            }
            // one particle has left the cell
            // the cell no longer has to remember the particle, and vice versa
            EventType::MoveOutOfCell(event) => {
                // update particles
                let p: &Rc<RefCell<Particle>> = event.execute(time);
                // for the cell from which the particle is leaving,
                //   1. cancel events related to this particle in the old cell
                //   2. remove the particle from the local particle list
                //   3. remove the cell from the cell list
                let cell_index: usize = event.cell_index;
                let cell: &Rc<RefCell<Cell>> = &cells[cell_index];
                cancel_events(p, cell, scheduler);
                cell.borrow_mut().remove(p);
                p.borrow_mut().remove(cell_index);
            }
            // NOTE: only when the direction is not periodic
            // update particle reflecting on the wall
            EventType::WallReflection(event) => {
                // update particles
                let p: &Rc<RefCell<Particle>> = event.execute(time);
                // cancel all events related to these two particles
                //   since their velocities are altered
                for &cell_index in p.borrow().cell_indices.iter() {
                    cancel_events(p, &cells[cell_index], scheduler);
                }
                // reschedule all events related to these two particles
                for &cell_index in p.borrow().cell_indices.iter() {
                    schedule_events(lengths, p, &cells[cell_index], scheduler);
                }
            }
            // update all particles to the desired time to synchronise for output
            // after this event the results are returned
            EventType::Synchronisation(event) => {
                event.execute(lengths, time, particles);
                // schedule next synchronisation
                insert_event(
                    Synchronisation::schedule(time + sync_rate),
                    &cells[0].borrow(),
                    scheduler,
                );
                break time;
            }
            // dummy event, nothing to do
            EventType::NotApplicable => {}
        }
    };
    time
}
