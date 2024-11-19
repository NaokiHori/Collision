mod myvec;
mod random;
mod simulator;

use myvec::MyVec;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::simulator::{Particle, Simulator, NDIMS};

fn main() {
    const SEED: f64 = 0.;
    let mut time: f64 = 0.;
    let time_max: f64 = 50.;
    let sync_rate: f64 = 1.;
    let lengths: [f64; NDIMS] = [800., 800.];
    let nparticles: usize = 65536;
    let mut simulator = Simulator::new(sync_rate, lengths, nparticles, SEED);
    loop {
        simulator.integrate();
        time += sync_rate;
        println!("time: {:8.2e}", time);
        let _ = save_particles(&lengths, &simulator);
        if time_max <= time {
            break;
        }
    }
}

fn save_particles(lengths: &[f64; NDIMS], simulator: &Simulator) -> Result<(), ()> {
    let particles: &Vec<Rc<RefCell<Particle>>> = simulator.get_particles();
    let canvas_size: [usize; 2] = [800, 800];
    let nitems: usize = canvas_size[0] * canvas_size[1];
    let mut pixels: Vec<u8> = vec![0u8; nitems * 3];
    for p in particles.iter() {
        let p: Ref<Particle> = p.borrow();
        let pos: &MyVec = &p.pos;
        let _radius: f64 = simulator::radius();
        let index: usize = (pos[1] / lengths[1] * canvas_size[1] as f64) as usize * canvas_size[0]
            + (pos[0] / lengths[0] * canvas_size[0] as f64) as usize;
        fit(p.val, &mut pixels[3 * index..3 * index + 3]);
    }
    {
        let fname: &str = "image.ppm";
        const MAGIC_NUMBER: &str = "P6";
        // open and prepare stream
        let file: std::fs::File = match std::fs::File::create(fname) {
            Ok(file) => file,
            Err(_) => {
                println!("failed to open file");
                return Err(());
            }
        };
        let mut stream: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
        // fwrite
        let _size: usize = match std::io::Write::write(
            &mut stream,
            format!(
                "{}\n{} {}\n255\n",
                MAGIC_NUMBER, &canvas_size[0], &canvas_size[1]
            )
            .as_bytes(),
        ) {
            Ok(size) => size,
            Err(_) => {
                println!("file write failed");
                return Err(());
            }
        };
        let _size: usize = match std::io::Write::write(&mut stream, &pixels) {
            Ok(size) => size,
            Err(_) => {
                println!("file write failed");
                return Err(());
            }
        };
    }
    {
        let mut xs = Vec::<u8>::new();
        let mut ys = Vec::<u8>::new();
        let mut vs = Vec::<u8>::new();
        for p in particles.iter() {
            let p: Ref<Particle> = p.borrow();
            let pos: &MyVec = &p.pos;
            let val: f64 = p.val;
            xs.extend_from_slice(&pos[0].to_le_bytes());
            ys.extend_from_slice(&pos[1].to_le_bytes());
            vs.extend_from_slice(&val.to_le_bytes());
        }
        {
            let file: std::fs::File = match std::fs::File::create("xs.bin") {
                Ok(file) => file,
                Err(_) => {
                    println!("failed to open file");
                    return Err(());
                }
            };
            let mut stream: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
            let _size: usize = match std::io::Write::write(&mut stream, &xs) {
                Ok(size) => size,
                Err(_) => {
                    println!("file write failed");
                    return Err(());
                }
            };
        }
        {
            let file: std::fs::File = match std::fs::File::create("ys.bin") {
                Ok(file) => file,
                Err(_) => {
                    println!("failed to open file");
                    return Err(());
                }
            };
            let mut stream: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
            let _size: usize = match std::io::Write::write(&mut stream, &ys) {
                Ok(size) => size,
                Err(_) => {
                    println!("file write failed");
                    return Err(());
                }
            };
        }
        {
            let file: std::fs::File = match std::fs::File::create("vs.bin") {
                Ok(file) => file,
                Err(_) => {
                    println!("failed to open file");
                    return Err(());
                }
            };
            let mut stream: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
            let _size: usize = match std::io::Write::write(&mut stream, &vs) {
                Ok(size) => size,
                Err(_) => {
                    println!("file write failed");
                    return Err(());
                }
            };
        }
    }
    Ok(())
}

fn fit(val: f64, rgb: &mut [u8]) {
    const PI: f64 = std::f64::consts::PI;
    let rgb_f64: [f64; 3] = [
        0.5 * (1. + (2. * PI * (val + 0. / 3.)).sin()),
        0.5 * (1. + (2. * PI * (val + 1. / 3.)).sin()),
        0.5 * (1. + (2. * PI * (val + 2. / 3.)).sin()),
    ];
    rgb[0] = (255. * rgb_f64[0]) as u8;
    rgb[1] = (255. * rgb_f64[1]) as u8;
    rgb[2] = (255. * rgb_f64[2]) as u8;
}
