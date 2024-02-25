use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::simulator::event::Event;
use crate::simulator::extrema::Extrema;
use crate::simulator::particle::Particle;
use crate::simulator::NDIMS;

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

fn get_neighbour_0_neg(ncells: &[usize; NDIMS], index: usize) -> usize {
    if 0 == index % ncells[0] {
        index + ncells[0] - 1
    } else {
        index - 1
    }
}

fn get_neighbour_1_neg(ncells: &[usize; NDIMS], index: usize) -> usize {
    if 0 == index / ncells[0] {
        index + ncells[0] * (ncells[1] - 1)
    } else {
        index - ncells[0]
    }
}

fn get_neighbour_0_pos(ncells: &[usize; NDIMS], index: usize) -> usize {
    if ncells[0] - 1 == index % ncells[0] {
        index + 1 - ncells[0]
    } else {
        index + 1
    }
}

fn get_neighbour_1_pos(ncells: &[usize; NDIMS], index: usize) -> usize {
    if ncells[1] - 1 == index / ncells[0] {
        index % ncells[0]
    } else {
        index + ncells[0]
    }
}

#[cfg(test)]
mod test_get_neighbour {
    use super::get_neighbour_0_neg;
    use super::get_neighbour_0_pos;
    use super::get_neighbour_1_neg;
    use super::get_neighbour_1_pos;
    use crate::simulator::NDIMS;
    #[test]
    fn case1() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(get_neighbour_0_neg(&ncells, 0), 2);
        assert_eq!(get_neighbour_0_neg(&ncells, 1), 0);
        assert_eq!(get_neighbour_0_neg(&ncells, 2), 1);
        assert_eq!(get_neighbour_0_neg(&ncells, 3), 5);
        assert_eq!(get_neighbour_0_neg(&ncells, 4), 3);
        assert_eq!(get_neighbour_0_neg(&ncells, 5), 4);
    }
    #[test]
    fn case2() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(get_neighbour_0_pos(&ncells, 0), 1);
        assert_eq!(get_neighbour_0_pos(&ncells, 1), 2);
        assert_eq!(get_neighbour_0_pos(&ncells, 2), 0);
        assert_eq!(get_neighbour_0_pos(&ncells, 3), 4);
        assert_eq!(get_neighbour_0_pos(&ncells, 4), 5);
        assert_eq!(get_neighbour_0_pos(&ncells, 5), 3);
    }
    #[test]
    fn case3() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(get_neighbour_1_neg(&ncells, 0), 3);
        assert_eq!(get_neighbour_1_neg(&ncells, 1), 4);
        assert_eq!(get_neighbour_1_neg(&ncells, 2), 5);
        assert_eq!(get_neighbour_1_neg(&ncells, 3), 0);
        assert_eq!(get_neighbour_1_neg(&ncells, 4), 1);
        assert_eq!(get_neighbour_1_neg(&ncells, 5), 2);
    }
    #[test]
    fn case4() {
        let ncells: [usize; NDIMS] = [3, 2];
        assert_eq!(get_neighbour_1_pos(&ncells, 0), 3);
        assert_eq!(get_neighbour_1_pos(&ncells, 1), 4);
        assert_eq!(get_neighbour_1_pos(&ncells, 2), 5);
        assert_eq!(get_neighbour_1_pos(&ncells, 3), 0);
        assert_eq!(get_neighbour_1_pos(&ncells, 4), 1);
        assert_eq!(get_neighbour_1_pos(&ncells, 5), 2);
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

pub fn init_cells(lengths: &[f64; NDIMS]) -> ([usize; NDIMS], Vec<Rc<RefCell<Cell>>>) {
    // decide number of cells
    let ncells: [usize; NDIMS] = [
        // require at least three cells for each direction
        3usize.max((lengths[0] / CELL_SIZE) as usize),
        3usize.max((lengths[1] / CELL_SIZE) as usize),
    ];
    // create cells
    let mut cells = Vec::<Rc<RefCell<Cell>>>::new();
    for index in 0..ncells[0] * ncells[1] {
        let i: usize = index % ncells[0];
        let j: usize = index / ncells[0];
        let bounds: [Extrema<f64>; NDIMS] = [
            Extrema {
                min: lengths[0] / ncells[0] as f64 * i as f64,
                max: lengths[0] / ncells[0] as f64 * (i + 1) as f64,
            },
            Extrema {
                min: lengths[1] / ncells[1] as f64 * j as f64,
                max: lengths[1] / ncells[1] as f64 * (j + 1) as f64,
            },
        ];
        let particles = Rc::new(RefCell::new(Vec::<Rc<RefCell<Particle>>>::new()));
        let events = Rc::new(RefCell::new(Vec::<Event>::new()));
        let positions: [CellPosition; NDIMS] = [
            if 0 == i {
                CellPosition::NegativeEdge
            } else if ncells[0] - 1 == i {
                CellPosition::PositiveEdge
            } else {
                CellPosition::Centre
            },
            if 0 == j {
                CellPosition::NegativeEdge
            } else if ncells[1] - 1 == j {
                CellPosition::PositiveEdge
            } else {
                CellPosition::Centre
            },
        ];
        let neighbours: [Extrema<usize>; NDIMS] = [
            Extrema::<usize> {
                min: get_neighbour_0_neg(&ncells, index),
                max: get_neighbour_0_pos(&ncells, index),
            },
            Extrema::<usize> {
                min: get_neighbour_1_neg(&ncells, index),
                max: get_neighbour_1_pos(&ncells, index),
            },
        ];
        let cell = Cell {
            index,
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
