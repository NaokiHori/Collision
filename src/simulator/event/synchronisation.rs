use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::cell::Cell;
use crate::simulator::particle::{find_minimum_distance, Particle};
use crate::simulator::scheduler::Scheduler;
use crate::simulator::Domain;

use super::{Event, EventType};

pub struct Synchronisation {
    /// The cell in which this event happens.
    cell: Rc<RefCell<Cell>>,
}

impl Synchronisation {
    pub fn schedule(time: f64, cell: &Rc<RefCell<Cell>>) -> Event {
        let event = Synchronisation { cell: cell.clone() };
        Event {
            time,
            eventtype: EventType::Synchronisation(event),
        }
    }

    pub fn execute(
        &self,
        domain: &Domain,
        time: f64,
        sync_rate: f64,
        particles: &[Rc<RefCell<Particle>>],
        scheduler: &mut Scheduler,
    ) {
        // update all particles
        for particle in particles.iter() {
            let mut p: RefMut<Particle> = particle.borrow_mut();
            p.pos = Particle::get_new_pos(domain, p.pos, p.vel, time - p.time);
            p.time = time;
        }
        // schedule next synchronisation
        super::insert_event(
            Synchronisation::schedule(time + sync_rate, &self.cell),
            &self.cell,
            scheduler,
        );
        // check stats, only for binary crate without optimisation
        if cfg!(debug_assertions) {
            check_energy(time, particles);
            check_distance(domain, time, particles);
        }
    }
}

#[allow(dead_code)]
fn check_energy(time: f64, particles: &[Rc<RefCell<Particle>>]) {
    // compute total energy
    let mut energy: f64 = 0.;
    for particle in particles.iter() {
        let p: Ref<Particle> = particle.borrow();
        energy += 0.5 * p.vel * p.vel;
    }
    let mut content = String::new();
    content += format!("{:+22.15e} {:+22.15e}", time, energy).as_str();
    save("energy.dat", &content);
}

#[allow(dead_code)]
fn check_distance(domain: &Domain, time: f64, particles: &[Rc<RefCell<Particle>>]) {
    let mut min: f64 = f64::MAX;
    for (n, p) in particles.iter().enumerate() {
        let p: Ref<Particle> = p.borrow();
        for q in particles[n + 1..].iter() {
            let q: Ref<Particle> = q.borrow();
            let mut dist: f64 = find_minimum_distance(domain, p.pos, q.pos);
            dist -= p.rad + q.rad;
            min = min.min(dist);
        }
    }
    let mut content = String::new();
    content += format!("{:+22.15e} {:+22.15e}", time, min).as_str();
    save("distance.dat", &content);
}

#[allow(dead_code)]
fn save(filename: &str, content: &str) {
    let mut file: std::fs::File = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("\"{}\": {}", filename, e);
            return;
        }
    };
    use std::io::Write;
    match writeln!(file, "{}", content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\"{}\": {}", filename, e)
        }
    };
}
