mod cell;
mod debug;
mod event;
mod extrema;
pub mod particle;
mod scheduler;
mod util;

use std::cell::RefCell;
use std::rc::Rc;

use cell::Cell;
pub use particle::Particle;
use scheduler::Scheduler;

pub const NDIMS: usize = 2usize;

pub struct Domain {
    lengths: [f64; NDIMS],
    periodicities: [bool; NDIMS],
}

pub struct Simulator {
    time: f64,
    domain: Domain,
    sync_rate: f64,
    particles: Vec<Rc<RefCell<Particle>>>,
    cells: Vec<Rc<RefCell<Cell>>>,
    scheduler: Scheduler,
}

impl Simulator {
    pub fn new(sync_rate: f64, lengths: [f64; NDIMS], nparticles: usize, seed: f64) -> Simulator {
        let domain = Domain {
            lengths,
            periodicities: {
                let mut periodicities = [true; NDIMS];
                if 1 < NDIMS {
                    periodicities[1] = false;
                }
                periodicities
            },
        };
        let time: f64 = 0.;
        let (ncells, cells): ([usize; NDIMS], Vec<Rc<RefCell<Cell>>>) = cell::init_cells(&domain);
        let particles: Vec<Rc<RefCell<Particle>>> =
            particle::init_particles(&domain, &ncells, &cells, nparticles, time, seed);
        let mut scheduler = Scheduler::new(&cells);
        event::init_events(&domain, &cells, &mut scheduler);
        Simulator {
            domain,
            time,
            sync_rate,
            particles,
            cells,
            scheduler,
        }
    }

    pub fn integrate(&mut self) {
        self.time = event::process_events(
            &self.domain,
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
