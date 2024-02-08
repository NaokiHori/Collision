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
        for particle in particles.iter() {
            let particle: Ref<Particle> = particle.borrow();
            const ARCS: [f64; 2] = [0., 2. * std::f64::consts::PI];
            let r: f64 = particle.rad;
            let x: f64 = particle.pos[0];
            let y: f64 = particle.pos[1];
            context.set_fill_style(&convert(particle.val));
            context.begin_path();
            context
                .arc(scale * x, scale * y, scale * r, ARCS[0], ARCS[1])
                .unwrap();
            context.fill();
        }
    }

    pub fn update_canvas_size(&self) {
        let canvas: &web_sys::HtmlCanvasElement = &self.canvas;
        let w: i32 = canvas.client_width();
        let h: i32 = canvas.client_height();
        canvas.set_width(w as u32);
        canvas.set_height(h as u32);
    }
}

fn convert(val: f64) -> JsValue {
    let rgbcoefs: [[f64; 3]; 5] = [
        [
            2.672303238499781e-01,
            5.015408860973969e-03,
            3.290548382054911e-01,
        ],
        [
            8.867281107764821e-01,
            1.415434679048477e+00,
            6.427369217396137e-01,
        ],
        [
            -6.777660845884058e+00,
            -8.089902514371242e-01,
            2.998258532949060e+00,
        ],
        [
            1.102198635856048e+01,
            7.296293729490473e-01,
            -9.057970794130403e+00,
        ],
        [
            -4.404685706758277e+00,
            -4.355228476501643e-01,
            5.230151793650696e+00,
        ],
    ];
    // fit polynomial
    let mut rgb: [f64; 3] = [0., 0., 0.];
    for (n, rgbcoef) in rgbcoefs.iter().enumerate() {
        for m in 0..3 {
            rgb[m] += rgbcoef[m] * val.powi(n as i32);
        }
    }
    // truncate
    for m in 0..3 {
        rgb[m] = if rgb[m] < 0. { 0. } else { rgb[m] };
        rgb[m] = if 1. < rgb[m] { 1. } else { rgb[m] };
    }
    let r: u8 = (255. * rgb[0]) as u8;
    let g: u8 = (255. * rgb[1]) as u8;
    let b: u8 = (255. * rgb[2]) as u8;
    let string = format!("#{r:02x}{g:02x}{b:02x}");
    JsValue::from_str(&string)
}
