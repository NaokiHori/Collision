mod drawer;
mod myvec;
mod random;
mod simulator;

use crate::drawer::Drawer;
use crate::simulator::{Simulator, NDIMS};

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Collision {
    simulator: crate::simulator::Simulator,
    drawer: crate::drawer::Drawer,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Collision {
    pub fn new(length: f64, nitems: usize, rate: f64, seed: f64) -> Collision {
        let lengths: [f64; NDIMS] = [length, length];
        let simulator = Simulator::new(rate, lengths, nitems, seed);
        let drawer = Drawer::new();
        Collision { simulator, drawer }
    }

    pub fn update(&mut self) {
        self.simulator.integrate();
        self.drawer
            .draw(self.simulator.get_lengths(), self.simulator.get_particles());
    }

    pub fn update_canvas_size(&self) {
        self.drawer.update_canvas_size();
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn init() {}
