use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::simulator::cell::Cell;
use crate::simulator::event::Event;

pub struct Scheduler {
    /// Number of items
    nitems: usize,
    /// Main minimum heap
    heap: Vec<Rc<RefCell<Cell>>>,
    /// Contravariant look-up table to find heap from cell index
    lookups: Vec<usize>,
}

impl Scheduler {
    /// Constructs a minimum heap.
    pub fn new(cells: &[Rc<RefCell<Cell>>]) -> Scheduler {
        let nitems: usize = cells.len();
        let mut heap = Vec::<Rc<RefCell<Cell>>>::new();
        let mut lookups = Vec::<usize>::new();
        for (n, cell) in cells.iter().enumerate() {
            heap.push(cell.clone());
            lookups.push(n);
        }
        Scheduler {
            nitems,
            heap,
            lookups,
        }
    }

    /// Returns cell which contains the coming event.
    pub fn get(&self) -> Rc<RefCell<Cell>> {
        if cfg!(debug_assertions) {
            self.validate();
        }
        self.heap[0].clone()
    }

    /// Updates heap.
    pub fn update(
        &mut self,
        cell_index: usize,
        event_bef: &Rc<RefCell<Event>>,
        event_aft: &Rc<RefCell<Event>>,
    ) {
        if event_bef.borrow().time < event_aft.borrow().time {
            self.downshift(self.lookups[cell_index]);
        } else {
            self.upshift(self.lookups[cell_index]);
        }
        if cfg!(debug_assertions) {
            self.validate();
        }
    }

    fn get_data(&self, heap: &[Rc<RefCell<Cell>>], index: usize) -> f64 {
        let nitems: usize = self.nitems;
        if nitems - 1 < index {
            return f64::MAX;
        }
        let cell: &Rc<RefCell<Cell>> = &heap[index];
        let cell: Ref<Cell> = cell.borrow();
        let events: &Rc<RefCell<Vec<Rc<RefCell<Event>>>>> = &cell.events;
        let events: Ref<Vec<Rc<RefCell<Event>>>> = events.borrow();
        let data: f64 = if events.is_empty() {
            f64::MAX
        } else {
            events[0].borrow().time
        };
        data
    }

    fn upshift(&mut self, mut n_c: usize) {
        let nitems: usize = self.nitems;
        while 0 < n_c && n_c < nitems {
            let heap: &Vec<Rc<RefCell<Cell>>> = &self.heap;
            let n_p: usize = parent(n_c);
            let data_c: f64 = self.get_data(heap, n_c);
            let data_p: f64 = self.get_data(heap, n_p);
            if data_c < data_p {
                let cell_index_c: usize = self.heap[n_c].borrow().index;
                let cell_index_p: usize = self.heap[n_p].borrow().index;
                self.heap.swap(n_c, n_p);
                self.lookups.swap(cell_index_c, cell_index_p);
                n_c = n_p;
            } else {
                break;
            }
        }
    }

    fn downshift(&mut self, mut n_p: usize) {
        let nitems: usize = self.nitems;
        while n_p < nitems {
            let heap: &Vec<Rc<RefCell<Cell>>> = &self.heap;
            let n_l: usize = lchild(n_p);
            let n_r: usize = rchild(n_p);
            let data_p: f64 = self.get_data(heap, n_p);
            let data_l: f64 = self.get_data(heap, n_l);
            let data_r: f64 = self.get_data(heap, n_r);
            if data_l < data_p && data_l <= data_r {
                let cell_index_l: usize = self.heap[n_l].borrow().index;
                let cell_index_p: usize = self.heap[n_p].borrow().index;
                self.heap.swap(n_l, n_p);
                self.lookups.swap(cell_index_l, cell_index_p);
                n_p = n_l;
            } else if data_r < data_p && data_r < data_l {
                let cell_index_p: usize = self.heap[n_p].borrow().index;
                let cell_index_r: usize = self.heap[n_r].borrow().index;
                self.heap.swap(n_p, n_r);
                self.lookups.swap(cell_index_p, cell_index_r);
                n_p = n_r;
            } else {
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn validate(&self) {
        let nitems: usize = self.nitems;
        let heap: &Vec<Rc<RefCell<Cell>>> = &self.heap;
        // for each child element, check its parent satisfies the requirement
        for n_c in 1..nitems {
            let n_p: usize = parent(n_c);
            let data_c: f64 = self.get_data(heap, n_c);
            let data_p: f64 = self.get_data(heap, n_p);
            if data_c < data_p {
                self.show();
                panic!("invalid heap");
            }
        }
        // for each parent element, check its children satisfy the requirement
        for n_p in 0..nitems {
            let n_l: usize = lchild(n_p);
            let n_r: usize = rchild(n_p);
            let data_p: f64 = self.get_data(heap, n_p);
            let data_l: f64 = self.get_data(heap, n_l);
            let data_r: f64 = self.get_data(heap, n_r);
            if (data_l < data_p && data_l <= data_r) || (data_r < data_p && data_r < data_l) {
                self.show();
                panic!("invalid heap");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        let nitems: usize = self.nitems;
        let heap: &Vec<Rc<RefCell<Cell>>> = &self.heap;
        for n in 0..nitems {
            let cell_index: usize = heap[n].borrow().index;
            let data: f64 = self.get_data(heap, n);
            println!("{:2}, c idx: {:2}, data: {:8.2e}", n, cell_index, data);
        }
    }
}

fn parent(n: usize) -> usize {
    (n - 1) / 2
}

fn lchild(n: usize) -> usize {
    2 * n + 1
}

fn rchild(n: usize) -> usize {
    2 * n + 2
}
