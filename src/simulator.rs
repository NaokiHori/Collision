mod cell;
mod debug;
mod event;
mod extrema;
pub mod particle;
mod scheduler;

use std::cell::RefCell;
use std::rc::Rc;

use cell::Cell;
pub use particle::Particle;
use scheduler::Scheduler;

// TODO: for now only 2 is available
pub const NDIMS: usize = 2;

pub const PERIODICITIES: [bool; NDIMS] = [true, false];

pub struct Simulator {
    lengths: [f64; NDIMS],
    time: f64,
    sync_rate: f64,
    particles: Vec<Rc<RefCell<Particle>>>,
    cells: Vec<Rc<RefCell<Cell>>>,
    scheduler: Scheduler,
}

impl Simulator {
    pub fn new(sync_rate: f64, lengths: [f64; NDIMS], nparticles: usize, seed: f64) -> Simulator {
        let time: f64 = 0.;
        let (ncells, cells): ([usize; NDIMS], Vec<Rc<RefCell<Cell>>>) = cell::init_cells(&lengths);
        let particles: Vec<Rc<RefCell<Particle>>> =
            particle::init_particles(&lengths, &ncells, &cells, nparticles, time, seed);
        let mut scheduler = Scheduler::new(&cells);
        event::init_events(&lengths, &cells, &mut scheduler);
        Simulator {
            lengths,
            time,
            sync_rate,
            particles,
            cells,
            scheduler,
        }
    }

    pub fn integrate(&mut self) {
        self.time = event::process_events(
            &self.lengths,
            &self.particles,
            &self.cells,
            &mut self.scheduler,
            self.sync_rate,
        );
    }

    pub fn get_particles(&self) -> &Vec<Rc<RefCell<Particle>>> {
        &self.particles
    }
}

pub fn radius() -> f64 {
    use particle::RADIUS;
    RADIUS
}
