use crate::display::display_object::DisplayObject;
use crate::graphics::shape::Shape;
use crate::utils::console_log;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::math::Matrix;
use std::cell::RefCell;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct Application {
    ctx: WebGl2RenderingContext,
    shapes: Vec<Rc<RefCell<dyn Shape>>>,
    dims: CanvasDimensions,
}

pub struct CanvasDimensions {
    pub width: f32,
    pub height: f32,
}

impl Application {
    pub fn new(ctx: &WebGl2RenderingContext, dims: CanvasDimensions) -> Self {
        let ctx = ctx.clone();
        //initialize context need to happens once
        ctx.viewport(
            0,
            0,
            ctx.drawing_buffer_width(),
            ctx.drawing_buffer_height(),
        );
        ctx.clear_color(0.8, 0.9, 1.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST);
        ctx.depth_func(WebGl2RenderingContext::LEQUAL);
        ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        Application {
            ctx,
            shapes: Vec::new(),
            dims,
        }
    }
    pub fn add_shape(&mut self, g: Rc<RefCell<dyn Shape>>) {
        self.shapes.push(g);
    }
    pub fn render_all(&mut self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        // self.gc();

        let mat = &Matrix::new();
        self.shapes.iter().for_each(|rc_shape| {
            let count = Rc::strong_count(rc_shape);
            let geom = rc_shape.borrow().get_geom();
            DisplayObject::new(&self.ctx, &geom, mat).draw();
        });
    }

    pub fn total_shapes(&self) -> usize {
        self.shapes.len()
    }

    pub fn gc(&mut self) {
        self.shapes = self
            .shapes
            .iter()
            .filter(|rc_shape| Rc::strong_count(rc_shape) > 1)
            .cloned()
            .collect();
    }
}
