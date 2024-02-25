use wasm_bindgen::prelude::*;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::simulator::particle::Particle;

pub struct Drawer {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl Drawer {
    pub fn new() -> Drawer {
        // Document object
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        // HTML canvas element
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id("my-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        // HTML canvas context object
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Drawer { canvas, context }
    }

    pub fn draw(&mut self, lengths: &[f64], particles: &[Rc<RefCell<Particle>>]) {
        let canvas: &web_sys::HtmlCanvasElement = &self.canvas;
        let context: &web_sys::CanvasRenderingContext2d = &self.context;
        // get canvas size
        let w: f64 = canvas.client_width() as f64;
        let h: f64 = canvas.client_height() as f64;
        // clean canvas
        context.clear_rect(0., 0., w, h);
        // map physical coordinate to screen coordinate
        // horizontal / vertical scaling
        // NOTE: assume squared domain
        let screen_size: f64 = w.min(h);
        let domain_size: f64 = {
            let lx: f64 = lengths[0];
            let ly: f64 = lengths[1];
            lx.min(ly)
        };
        let scale: f64 = screen_size / domain_size;
        // draw circles
        const ARCS: [f64; 2] = [0., 2. * std::f64::consts::PI];
        context.set_fill_style(&JsValue::from_str("#8888ff"));
        context.begin_path();
        for particle in particles.iter() {
            let particle: Ref<Particle> = particle.borrow();
            let r: f64 = scale * particle.rad;
            let x: f64 = scale * particle.pos[0];
            let y: f64 = scale * particle.pos[1];
            context.move_to(x, y);
            context.arc(x, y, r, ARCS[0], ARCS[1]).unwrap();
        }
        context.fill();
    }

    pub fn update_canvas_size(&self) {
        let canvas: &web_sys::HtmlCanvasElement = &self.canvas;
        let w: i32 = canvas.client_width();
        let h: i32 = canvas.client_height();
        canvas.set_width(w as u32);
        canvas.set_height(h as u32);
    }
}
