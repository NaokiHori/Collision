use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::myvec::MyVec;
use crate::random::Random;
use crate::simulator::cell::Cell;
use crate::simulator::extrema::Extrema;
use crate::simulator::{NDIMS, PERIODICITIES};

/// Particle radius, to be consistent with the cell size.
pub const RADIUS: f64 = 0.5f64;

pub struct Particle {
    pub index: usize,
    pub rad: f64,
    pub pos: MyVec,
    pub vel: MyVec,
    pub time: f64,
    pub cells: Vec<Rc<RefCell<Cell>>>,
    pub val: f64,
}

#[allow(dead_code)]
fn check_duplication(p: &Particle, cells: &[Rc<RefCell<Cell>>]) {
    // check no duplication
    for (n, c0) in cells.iter().enumerate() {
        for c1 in cells[n + 1..].iter() {
            if Rc::ptr_eq(c0, c1) {
                panic!(
                    "cell {} is duplicated for particle {} at ({}, {})",
                    c0.borrow().index,
                    p.index,
                    p.pos[0],
                    p.pos[1]
                );
            }
        }
    }
}

impl Particle {
    pub fn append(&mut self, cell: &Rc<RefCell<Cell>>) {
        self.cells.push(cell.clone());
        if cfg!(debug_assertions) {
            check_duplication(self, &self.cells);
        }
    }

    pub fn remove(&mut self, cell: &Rc<RefCell<Cell>>) {
        let cells: &mut Vec<Rc<RefCell<Cell>>> = &mut self.cells;
        let position: usize = cells.iter().position(|c| Rc::ptr_eq(c, cell)).unwrap();
        cells.remove(position);
        if cfg!(debug_assertions) {
            check_duplication(self, &self.cells);
        }
    }

    pub fn get_new_pos(lengths: &[f64; NDIMS], pos: MyVec, vel: MyVec, dt: f64) -> MyVec {
        // x^{n+1} = x^n + v * dt
        let mut new_pos: MyVec = pos + vel * dt;
        // correct periodicity
        for dim in 0..NDIMS {
            if new_pos[dim] < 0. {
                new_pos[dim] += lengths[dim];
            } else if lengths[dim] < new_pos[dim] {
                new_pos[dim] -= lengths[dim];
            }
        }
        new_pos
    }
}

fn from_p_to_c(
    lengths: &[f64; NDIMS],
    ncells: &[usize; NDIMS],
    rad: f64,
    pos: &MyVec,
) -> Vec<usize> {
    fn kernel(length: f64, ncells: usize, min: f64, max: f64, indices: &mut Vec<usize>) {
        let min: f64 = min / (length / ncells as f64);
        let max: f64 = max / (length / ncells as f64) + 1.;
        // cap for safety
        let min: f64 = min.max(0.);
        let max: f64 = max.min(ncells as f64);
        let min: usize = min as usize;
        let max: usize = max as usize;
        for n in min..max {
            indices.push(n);
        }
    }
    let mut indices = vec![<Vec<usize>>::new(); NDIMS];
    for dim in 0..NDIMS {
        let length: f64 = lengths[dim];
        let n: usize = ncells[dim];
        let min: f64 = pos[dim] - rad;
        let max: f64 = pos[dim] + rad;
        let index: &mut Vec<usize> = &mut indices[dim];
        if min < 0. {
            // negative-edge side
            kernel(length, n, 0., max, index);
            // positive-edge side
            kernel(length, n, length + min, length, index);
        } else if length < max {
            // negative-edge side
            kernel(length, n, 0., max - length, index);
            // positive-edge side
            kernel(length, n, min, length, index);
        } else {
            kernel(length, n, min, max, index);
        }
    }
    // TODO: generalise
    let mut cell_indices = Vec::<usize>::new();
    for index1 in indices[1].iter() {
        for index0 in indices[0].iter() {
            cell_indices.push(index0 + ncells[0] * index1);
        }
    }
    cell_indices
}

#[cfg(test)]
mod test_from_p_to_c {
    use super::from_p_to_c as func;
    use crate::myvec::MyVec;
    use crate::simulator::NDIMS;
    const LENGTHS: [f64; NDIMS] = [8., 8.];
    const CELL_SIZE: f64 = 2.;
    const NCELLS: [usize; NDIMS] = [
        (LENGTHS[0] / CELL_SIZE) as usize,
        (LENGTHS[1] / CELL_SIZE) as usize,
    ];
    #[test]
    fn case1() {
        let rad: f64 = 0.5;
        let pos: MyVec = MyVec::new([0., 0.6]);
        assert_eq!(func(&LENGTHS, &NCELLS, rad, &pos), vec![0, NCELLS[0] - 1]);
    }
    #[test]
    fn case2() {
        let rad: f64 = 0.5;
        let pos: MyVec = MyVec::new([0., 0.4]);
        assert_eq!(
            func(&LENGTHS, &NCELLS, rad, &pos),
            vec![
                0,
                NCELLS[0] - 1,
                (NCELLS[1] - 1) * NCELLS[0],
                NCELLS[1] * NCELLS[0] - 1
            ]
        );
    }
    #[test]
    fn case3() {
        let rad: f64 = 0.5;
        let pos: MyVec = MyVec::new([LENGTHS[0], 0.6]);
        assert_eq!(func(&LENGTHS, &NCELLS, rad, &pos), vec![0, NCELLS[0] - 1,]);
    }
    #[test]
    fn case4() {
        let rad: f64 = 0.5;
        let pos: MyVec = MyVec::new([0., 0.4]);
        assert_eq!(
            func(&LENGTHS, &NCELLS, rad, &pos),
            vec![
                0,
                NCELLS[0] - 1,
                (NCELLS[1] - 1) * NCELLS[0],
                NCELLS[1] * NCELLS[0] - 1
            ]
        );
    }
    #[test]
    fn case5() {
        let rad: f64 = 0.49 * LENGTHS[0];
        let pos: MyVec = MyVec::new([0.5 * LENGTHS[0], 0.5 * LENGTHS[1]]);
        assert_eq!(
            func(&LENGTHS, &NCELLS, rad, &pos).len(),
            NCELLS[0] * NCELLS[1]
        );
    }
    #[test]
    fn case6() {
        let rad: f64 = 0.49 * LENGTHS[0];
        let pos: MyVec = MyVec::new([0., 0.]);
        assert_eq!(
            func(&LENGTHS, &NCELLS, rad, &pos).len(),
            NCELLS[0] * NCELLS[1]
        );
    }
}

/// Finds minimum distance between two points, taking periodicity into account.
pub fn find_minimum_distance(lengths: &[f64; NDIMS], pos0: MyVec, pos1: MyVec) -> f64 {
    let dpos: MyVec = pos1 - pos0;
    let mut dist: f64 = 0.;
    for dim in 0..NDIMS {
        let d: f64 = dpos[dim].abs();
        let d: f64 = d.min((dpos[dim] - lengths[dim]).abs());
        let d: f64 = d.min((dpos[dim] + lengths[dim]).abs());
        dist += d.powi(2);
    }
    dist.sqrt()
}

#[cfg(test)]
mod test_find_minimum_distance {
    use super::find_minimum_distance;
    use crate::myvec::MyVec;
    use crate::simulator::NDIMS;
    #[test]
    fn case1() {
        // normal case, 3:4:5
        const LENGTHS: [f64; NDIMS] = [32., 32.];
        let pos0: MyVec = MyVec::new([1., 2.]);
        let pos1: MyVec = MyVec::new([4., 6.]);
        assert_eq!(find_minimum_distance(&LENGTHS, pos0, pos1), 5.);
    }
    #[test]
    fn case2() {
        // case with periodicity
        const LENGTHS: [f64; NDIMS] = [32., 32.];
        let pos0: MyVec = MyVec::new([0., 0.]);
        let pos1: MyVec = MyVec::new([0., LENGTHS[1]]);
        assert_eq!(find_minimum_distance(&LENGTHS, pos0, pos1), 0.);
    }
    #[test]
    fn case3() {
        // case with periodicity
        const LENGTHS: [f64; NDIMS] = [32., 32.];
        let pos0: MyVec = MyVec::new([0., 0.]);
        let pos1: MyVec = MyVec::new([LENGTHS[0], LENGTHS[1]]);
        assert_eq!(find_minimum_distance(&LENGTHS, pos0, pos1), 0.);
    }
}

pub fn init_particles(
    lengths: &[f64; NDIMS],
    ncells: &[usize; NDIMS],
    cells: &[Rc<RefCell<Cell>>],
    mut nitems: usize,
    time: f64,
    seed: f64,
) -> Vec<Rc<RefCell<Particle>>> {
    let rad: f64 = RADIUS;
    nitems = {
        let max_vfrac: f64 = 0.4;
        let max_nitems: f64 =
            (lengths[0] * lengths[1] * max_vfrac) / (std::f64::consts::PI * rad.powi(2));
        let max_nitems: usize = max_nitems as usize;
        max_nitems.min(nitems)
    };
    // request the cell sizes are larger than twice the particle diameters
    for cell in cells.iter() {
        for dim in 0..NDIMS {
            let bounds: &Extrema<f64> = &cell.borrow().bounds[dim];
            let d: f64 = bounds.max - bounds.min;
            if d <= 4. * rad {
                panic!(
                    "cell size {:+.2e} should be larger than the particle diameter {:+.2e}",
                    d,
                    2. * rad
                );
            }
        }
    }
    let mut rng = Random::new((seed * f64::MAX) as u64);
    let mut particles = Vec::<Rc<RefCell<Particle>>>::new();
    for index in 0..nitems {
        // find a proper position for a particle without overlapping
        //   with the other particles already defined
        let (pos, cell_indices): (MyVec, Vec<usize>) = 'find_no_overlap: loop {
            // choose position randomly
            let pos0: MyVec = {
                let x: f64 = {
                    let min: f64 = if PERIODICITIES[0] { 0. } else { rad };
                    let max: f64 = if PERIODICITIES[0] {
                        lengths[0]
                    } else {
                        lengths[0] - rad
                    };
                    rng.gen_range(min, max)
                };
                let y: f64 = {
                    let min: f64 = if PERIODICITIES[1] { 0. } else { rad };
                    let max: f64 = if PERIODICITIES[1] {
                        lengths[1]
                    } else {
                        lengths[1] - rad
                    };
                    rng.gen_range(min, max)
                };
                MyVec::new([x, y])
            };
            // get all cells to which this particle will belong
            let cell_indices: Vec<usize> = from_p_to_c(lengths, ncells, rad, &pos0);
            for &index in cell_indices.iter() {
                // check overlap for all particles which share the same cell
                let cell: Ref<Cell> = cells[index].borrow();
                let ps: Ref<Vec<Rc<RefCell<Particle>>>> = cell.particles.borrow();
                for p in ps.iter() {
                    let p: Ref<Particle> = p.borrow();
                    let pos1: MyVec = p.pos;
                    let dist: f64 = find_minimum_distance(lengths, pos0, pos1);
                    if dist < 2. * rad {
                        continue 'find_no_overlap;
                    }
                }
            }
            break (pos0, cell_indices);
        };
        let vel = MyVec::new([rng.gen_range(-1., 1.), rng.gen_range(-1., 1.)]);
        let val: f64 = if pos[0] / lengths[0] < pos[1] / lengths[1] {
            1.
        } else {
            0.
        };
        let particle = Rc::new(RefCell::new(Particle {
            index,
            rad,
            pos,
            vel,
            time,
            cells: {
                let mut cs = Vec::<Rc<RefCell<Cell>>>::new();
                for &cell_index in cell_indices.iter() {
                    let cell: Rc<RefCell<Cell>> = cells[cell_index].clone();
                    cs.push(cell);
                }
                cs
            },
            val,
        }));
        // add particle to the local list for each cell
        for &cell_index in cell_indices.iter() {
            let cell: Ref<Cell> = cells[cell_index].borrow();
            let mut ps: RefMut<Vec<Rc<RefCell<Particle>>>> = cell.particles.borrow_mut();
            ps.push(particle.clone());
        }
        // append to the main vector including all particles
        particles.push(particle);
    }
    // enforce zero net momentum
    {
        let mut mean = MyVec::new([0.; NDIMS]);
        for p in particles.iter() {
            let p: Ref<Particle> = p.borrow();
            mean = mean + p.vel;
        }
        mean = mean / particles.len() as f64;
        for p in particles.iter_mut() {
            let mut p: RefMut<Particle> = p.borrow_mut();
            p.vel = p.vel - mean;
        }
    }
    particles
}
