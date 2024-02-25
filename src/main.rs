mod myvec;
mod random;
mod simulator;

use crate::simulator::{Simulator, NDIMS};

fn main() {
    const SEED: f64 = 0.;
    let mut time: f64 = 0.;
    let time_max: f64 = 200.;
    let sync_rate: f64 = 1.;
    let lengths: [f64; NDIMS] = [16., 16.];
    let nparticles: usize = 128;
    let mut simulator = Simulator::new(sync_rate, lengths, nparticles, SEED);
    let ncells: &[usize; NDIMS] = simulator.get_ncells();
    println!("number of cells: ({}, {})", ncells[0], ncells[1]);
    loop {
        simulator.integrate();
        time += sync_rate;
        println!("time: {:8.2e}", time);
        if time_max <= time {
            break;
        }
    }
}
