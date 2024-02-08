mod myvec;
mod random;
mod simulator;

use crate::simulator::{Simulator, NDIMS};

fn main() {
    let mut time: f64 = 0.;
    let time_max: f64 = 100.;
    let sync_rate: f64 = 1.;
    let lengths: [f64; NDIMS] = [32., 32.];
    let nparticles: usize = 256;
    let mut simulator = Simulator::new(sync_rate, lengths, nparticles);
    let ncells: &[usize; NDIMS] = simulator.get_ncells();
    println!("number of cells: ({}, {})", ncells[0], ncells[1]);
    loop {
        simulator.integrate();
        time += sync_rate;
        println!("time: {:8.2e}", time);
        if time_max < time {
            break;
        }
    }
}
