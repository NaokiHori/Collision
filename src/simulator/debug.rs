use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::simulator::cell::Cell;
use crate::simulator::particle::Particle;

/// Checks if the particle knows which cells it belongs, and vice versa.
#[allow(dead_code)]
pub fn check_recognition(p: &Rc<RefCell<Particle>>, c0: &Rc<RefCell<Cell>>) {
    // particle -> cell check
    {
        let mut is_included: bool = false;
        let qs: &Rc<RefCell<Vec<Rc<RefCell<Particle>>>>> = &c0.borrow().particles;
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
                c0.borrow().index,
                p.borrow().index
            );
        }
    }
    // cell -> particle check
    {
        let mut is_included: bool = false;
        let cells: &Vec<Rc<RefCell<Cell>>> = &p.borrow().cells;
        for c1 in cells.iter() {
            if Rc::ptr_eq(c0, c1) {
                is_included = true;
                break;
            }
        }
        if !is_included {
            panic!(
                "particle {} does not recognise it belongs to the cell {}",
                p.borrow().index,
                c0.borrow().index
            );
        }
    }
}
