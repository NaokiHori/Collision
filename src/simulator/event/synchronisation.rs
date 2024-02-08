use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::particle::{find_minimum_distance, Particle};
use crate::simulator::NDIMS;

use super::{Event, EventType, Synchronisation};

pub fn schedule(time: f64) -> Event {
    let event = Synchronisation {};
    Event {
        time,
        eventtype: EventType::Synchronisation(event),
    }
}

pub fn execute(lengths: &[f64; NDIMS], time: f64, particles: &[Rc<RefCell<Particle>>]) {
    // update all particles
    for particle in particles.iter() {
        let mut p: RefMut<Particle> = particle.borrow_mut();
        p.pos = Particle::get_new_pos(lengths, p.pos, p.vel, time - p.time);
        p.time = time;
    }
    check_energy(time, particles);
    if cfg!(debug_assertions) {
        check_distance(lengths, time, particles);
    }
}

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
fn check_distance(lengths: &[f64; NDIMS], time: f64, particles: &[Rc<RefCell<Particle>>]) {
    let mut min: f64 = f64::MAX;
    for (n, p) in particles.iter().enumerate() {
        let p: Ref<Particle> = p.borrow();
        for q in particles[n + 1..].iter() {
            let q: Ref<Particle> = q.borrow();
            let mut dist: f64 = find_minimum_distance(lengths, p.pos, q.pos);
            dist -= p.rad + q.rad;
            min = min.min(dist);
        }
    }
    let mut content = String::new();
    content += format!("{:+22.15e} {:+22.15e}", time, min).as_str();
    save("distance.dat", &content);
}

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
