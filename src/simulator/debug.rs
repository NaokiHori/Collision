use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::simulator::cell::Cell;
use crate::simulator::particle::Particle;

/// Checks if the particle knows which cells it belongs, and vice versa.
#[allow(dead_code)]
pub fn check_recognition(p: &Rc<RefCell<Particle>>, cell: &Ref<Cell>) {
    // particle -> cell check
    let mut is_included: bool = false;
    let qs: &Rc<RefCell<Vec<Rc<RefCell<Particle>>>>> = &cell.particles;
    let qs: Ref<Vec<Rc<RefCell<Particle>>>> = qs.borrow();
    for q in qs.iter() {
        if Rc::ptr_eq(p, q) {
            is_included = true;
            break;
        }
    }
    if !is_included {
        panic!(
            "cell {} does not recognise it contains the particle {}",
            cell.index,
            p.borrow().index
        );
    }
    // cell -> particle check
    let mut is_included: bool = false;
    let cell_indices: &Vec<usize> = &p.borrow().cell_indices;
    for &cell_index in cell_indices.iter() {
        if cell.index == cell_index {
            is_included = true;
            break;
        }
    }
    if !is_included {
        panic!(
            "particle {} does not recognise it belongs to the cell {}",
            p.borrow().index,
            cell.index
        );
    }
}
