use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::event::Event;
use crate::simulator::extrema::Extrema;
use crate::simulator::particle::Particle;
use crate::simulator::util::vec_to_array;
use crate::simulator::{Domain, NDIMS};

/// Reference cell size.
///
/// N.B.: This is a "typical" size of a cell and is not necessarily the exact size.
///       For safety give more than 4 times larger than the radius of particles.
const CELL_SIZE: f64 = 3.;

/// Used to take into account the periodicity.
pub enum CellPosition {
    NegativeEdge,
    PositiveEdge,
    Centre,
}

pub struct Cell {
    pub index: usize,
    pub bounds: [Extrema<f64>; NDIMS],
    pub particles: Rc<RefCell<Vec<Rc<RefCell<Particle>>>>>,
    pub events: Rc<RefCell<Vec<Event>>>,
    pub positions: [CellPosition; NDIMS],
    pub neighbours: [Extrema<usize>; NDIMS],
}

#[allow(dead_code)]
fn check_duplication(cell_index: usize, particles: &Ref<Vec<Rc<RefCell<Particle>>>>) {
    // check no duplication
    for (n, p) in particles.iter().enumerate() {
        for q in particles[n + 1..].iter() {
            if Rc::ptr_eq(p, q) {
                let p: Ref<Particle> = p.borrow();
                panic!(
                    "particle {} at ({}, {}) is duplicated in cell {}",
                    p.index, p.pos[0], p.pos[1], cell_index
                );
            }
        }
    }
}

fn get_indices(ndims: usize, ncells: &[usize], mut index: usize) -> Vec<usize> {
    let mut indices = Vec::<usize>::with_capacity(ndims);
    for &dim_size in ncells.iter() {
        let current_index = index % dim_size;
        indices.push(current_index);
        index = (index - current_index) / dim_size;
    }
    indices
}

fn get_index(ndims: usize, ncells: &[usize], indices: &[usize]) -> usize {
    if ndims != ncells.len() {
        panic!("invalid length: {}", ncells.len());
    }
    if ndims != indices.len() {
        panic!("invalid length: {}", indices.len());
    }
    let mut index = 0;
    for (dim, &dim_size) in ncells.iter().enumerate().rev() {
        index *= dim_size;
        index += indices[dim];
    }
    index
}

fn get_neighbour(ncells: &[usize], dim: usize, index: usize) -> Extrema<usize> {
    let indices: Vec<usize> = get_indices(NDIMS, ncells, index);
    let m_indices: Vec<usize> = indices
        .iter()
        .enumerate()
        .map(|(d, &i)| {
            if d == dim {
                if 0 == i {
                    ncells[d] - 1
                } else {
                    i - 1
                }
            } else {
                i
            }
        })
        .collect();
    let p_indices: Vec<usize> = indices
        .iter()
        .enumerate()
        .map(|(d, &i)| {
            if d == dim {
                if ncells[d] - 1 == i {
                    0
                } else {
                    i + 1
                }
            } else {
                i
            }
        })
        .collect();
    Extrema::<usize> {
        min: get_index(NDIMS, ncells, &m_indices),
        max: get_index(NDIMS, ncells, &p_indices),
    }
}

impl Cell {
    pub fn append(&mut self, p: &Rc<RefCell<Particle>>) {
        {
            let mut particles: RefMut<Vec<Rc<RefCell<Particle>>>> = self.particles.borrow_mut();
            particles.push(p.clone());
        }
        if cfg!(debug_assertions) {
            check_duplication(self.index, &self.particles.borrow());
        }
    }

    pub fn remove(&mut self, p: &Rc<RefCell<Particle>>) {
        {
            let mut particles: RefMut<Vec<Rc<RefCell<Particle>>>> = self.particles.borrow_mut();
            let pos: usize = particles.iter().position(|q| Rc::ptr_eq(p, q)).unwrap();
            particles.remove(pos);
        }
        if cfg!(debug_assertions) {
            check_duplication(self.index, &self.particles.borrow());
        }
    }
}

pub fn init_cells(domain: &Domain) -> ([usize; NDIMS], Vec<Rc<RefCell<Cell>>>) {
    let lengths: &[f64; NDIMS] = &domain.lengths;
    // decide number of cells
    // require at least three cells for each direction
    let ncells: [usize; NDIMS] = {
        let mut ncells = Vec::<usize>::with_capacity(NDIMS);
        for dim in 0..NDIMS {
            ncells.push(3usize.max((lengths[dim] / CELL_SIZE) as usize));
        }
        vec_to_array(ncells)
    };
    // create cells
    let mut cells = Vec::<Rc<RefCell<Cell>>>::new();
    for n in 0..ncells.iter().product() {
        let indices: [usize; NDIMS] = vec_to_array(get_indices(NDIMS, &ncells, n));
        let bounds: [Extrema<f64>; NDIMS] = {
            let mut bounds = Vec::<Extrema<f64>>::with_capacity(NDIMS);
            for dim in 0..NDIMS {
                bounds.push(Extrema::<f64> {
                    min: lengths[dim] / ncells[dim] as f64 * (indices[dim] + 0) as f64,
                    max: lengths[dim] / ncells[dim] as f64 * (indices[dim] + 1) as f64,
                });
            }
            vec_to_array::<Extrema<f64>>(bounds)
        };
        let particles = Rc::new(RefCell::new(Vec::<Rc<RefCell<Particle>>>::new()));
        let events = Rc::new(RefCell::new(Vec::<Event>::new()));
        let positions: [CellPosition; NDIMS] = {
            let mut positions = Vec::<CellPosition>::with_capacity(NDIMS);
            for dim in 0..NDIMS {
                positions.push(if 0 == indices[dim] {
                    CellPosition::NegativeEdge
                } else if ncells[dim] - 1 == indices[dim] {
                    CellPosition::PositiveEdge
                } else {
                    CellPosition::Centre
                })
            }
            vec_to_array(positions)
        };
        let neighbours: [Extrema<usize>; NDIMS] = {
            let mut neighbours = Vec::<Extrema<usize>>::with_capacity(NDIMS);
            for dim in 0..NDIMS {
                neighbours.push(get_neighbour(&ncells, dim, n));
            }
            vec_to_array::<Extrema<usize>>(neighbours)
        };
        let cell = Cell {
            index: n,
            bounds,
            particles,
            events,
            positions,
            neighbours,
        };
        cells.push(Rc::new(RefCell::new(cell)));
    }
    (ncells, cells)
}

#[cfg(test)]
mod test_get_indices {
    use super::get_indices;

    #[test]
    fn case1() {
        const NDIMS: usize = 2usize;
        let ncells: [usize; NDIMS] = [2, 3];
        assert_eq!(get_indices(NDIMS, &ncells, 0), [0, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 1), [1, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 2), [0, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 3), [1, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 4), [0, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 5), [1, 2]);
    }

    #[test]
    fn case2() {
        const NDIMS: usize = 3usize;
        let ncells: [usize; NDIMS] = [3, 2, 4];
        assert_eq!(get_indices(NDIMS, &ncells, 0), [0, 0, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 1), [1, 0, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 2), [2, 0, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 3), [0, 1, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 4), [1, 1, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 5), [2, 1, 0]);
        assert_eq!(get_indices(NDIMS, &ncells, 6), [0, 0, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 7), [1, 0, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 8), [2, 0, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 9), [0, 1, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 10), [1, 1, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 11), [2, 1, 1]);
        assert_eq!(get_indices(NDIMS, &ncells, 12), [0, 0, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 13), [1, 0, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 14), [2, 0, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 15), [0, 1, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 16), [1, 1, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 17), [2, 1, 2]);
        assert_eq!(get_indices(NDIMS, &ncells, 18), [0, 0, 3]);
        assert_eq!(get_indices(NDIMS, &ncells, 19), [1, 0, 3]);
        assert_eq!(get_indices(NDIMS, &ncells, 20), [2, 0, 3]);
        assert_eq!(get_indices(NDIMS, &ncells, 21), [0, 1, 3]);
        assert_eq!(get_indices(NDIMS, &ncells, 22), [1, 1, 3]);
        assert_eq!(get_indices(NDIMS, &ncells, 23), [2, 1, 3]);
    }
}

#[cfg(test)]
mod test_get_index {
    use super::get_index;

    #[test]
    fn case1() {
        const NDIMS: usize = 2usize;
        let ncells: [usize; NDIMS] = [2, 3];
        let ncells = ncells.to_vec();
        assert_eq!(get_index(NDIMS, &ncells, &[0, 0]), 0);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 0]), 1);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 1]), 2);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 1]), 3);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 2]), 4);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 2]), 5);
    }

    #[test]
    fn case2() {
        const NDIMS: usize = 3usize;
        let ncells: [usize; NDIMS] = [3, 2, 4];
        assert_eq!(get_index(NDIMS, &ncells, &[0, 0, 0]), 0);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 0, 0]), 1);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 0, 0]), 2);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 1, 0]), 3);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 1, 0]), 4);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 1, 0]), 5);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 0, 1]), 6);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 0, 1]), 7);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 0, 1]), 8);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 1, 1]), 9);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 1, 1]), 10);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 1, 1]), 11);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 0, 2]), 12);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 0, 2]), 13);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 0, 2]), 14);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 1, 2]), 15);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 1, 2]), 16);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 1, 2]), 17);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 0, 3]), 18);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 0, 3]), 19);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 0, 3]), 20);
        assert_eq!(get_index(NDIMS, &ncells, &[0, 1, 3]), 21);
        assert_eq!(get_index(NDIMS, &ncells, &[1, 1, 3]), 22);
        assert_eq!(get_index(NDIMS, &ncells, &[2, 1, 3]), 23);
    }
}

#[cfg(test)]
mod test_get_neighbour {
    use super::get_neighbour;
    use crate::simulator::extrema::Extrema;
    use crate::simulator::NDIMS;

    #[test]
    fn neighbor_x() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(
            get_neighbour(&ncells, 0, 0),
            Extrema::<usize> { min: 2, max: 1 }
        );
        assert_eq!(
            get_neighbour(&ncells, 0, 1),
            Extrema::<usize> { min: 0, max: 2 }
        );
        assert_eq!(
            get_neighbour(&ncells, 0, 2),
            Extrema::<usize> { min: 1, max: 0 }
        );
        assert_eq!(
            get_neighbour(&ncells, 0, 3),
            Extrema::<usize> { min: 5, max: 4 }
        );
        assert_eq!(
            get_neighbour(&ncells, 0, 4),
            Extrema::<usize> { min: 3, max: 5 }
        );
        assert_eq!(
            get_neighbour(&ncells, 0, 5),
            Extrema::<usize> { min: 4, max: 3 }
        );
    }

    #[test]
    fn neighbor_y() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(
            get_neighbour(&ncells, 1, 0),
            Extrema::<usize> { min: 3, max: 3 }
        );
        assert_eq!(
            get_neighbour(&ncells, 1, 1),
            Extrema::<usize> { min: 4, max: 4 }
        );
        assert_eq!(
            get_neighbour(&ncells, 1, 2),
            Extrema::<usize> { min: 5, max: 5 }
        );
        assert_eq!(
            get_neighbour(&ncells, 1, 3),
            Extrema::<usize> { min: 0, max: 0 }
        );
        assert_eq!(
            get_neighbour(&ncells, 1, 4),
            Extrema::<usize> { min: 1, max: 1 }
        );
        assert_eq!(
            get_neighbour(&ncells, 1, 5),
            Extrema::<usize> { min: 2, max: 2 }
        );
    }
}
