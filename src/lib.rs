mod myvec;
mod random;
mod simulator;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::simulator::{Simulator, NDIMS};

#[wasm_bindgen]
pub struct Collision {
    simulator: crate::simulator::Simulator,
    positions: Vec<f32>,
    temperatures: Vec<f32>,
}

#[wasm_bindgen]
impl Collision {
    #[wasm_bindgen(constructor)]
    pub fn new(length: f64, nitems: usize, rate: f64, seed: f64) -> Collision {
        let lengths: [f64; NDIMS] = [length, length];
        let simulator = Simulator::new(rate, lengths, nitems, seed);
        let positions = vec![0f32; nitems * NDIMS];
        let temperatures = vec![0f32; nitems];
        Collision {
            simulator,
            positions,
            temperatures,
        }
    }

    pub fn positions(&self) -> *const f32 {
        self.positions.as_ptr()
    }

    pub fn temperatures(&self) -> *const f32 {
        self.temperatures.as_ptr()
    }

    pub fn update(&mut self) {
        use crate::simulator::Particle;
        self.simulator.integrate();
        let particles: &[Rc<RefCell<Particle>>] = self.simulator.get_particles();
        let positions: &mut [f32] = &mut self.positions;
        let temperatures: &mut [f32] = &mut self.temperatures;
        for (index, particle) in particles.iter().enumerate() {
            let particle: Ref<Particle> = particle.borrow();
            positions[2 * index] = particle.pos[0] as f32;
            positions[2 * index + 1] = particle.pos[1] as f32;
            temperatures[index] = particle.val as f32;
        }
    }
}

#[wasm_bindgen]
pub fn radius() -> f64 {
    simulator::radius()
}

#[wasm_bindgen(start)]
pub fn init() {}
