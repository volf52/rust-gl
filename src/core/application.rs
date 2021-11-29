use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::graphics::{Container, Shape};
use crate::math::Matrix;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct Application {
    pub ctx: WebGl2RenderingContext,
    pub proj_mat: Matrix,
    pub stage: Container,
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

        let proj_mat = Matrix::projection(dims.width, dims.height);

        Application {
            ctx,
            proj_mat,
            stage: Container::default(),
        }
    }

    pub fn render(&self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        self.stage.render(self, &Matrix::new());
    }

    pub fn add_container(&mut self, container: &Container) {
        self.stage.add_container(container);
    }

    pub fn add_shape(&mut self, shape: &impl Shape) {
        self.stage.add_shape(shape);
    }

    // pub fn gc(&mut self) {
    //     self.shapes = self
    //         .shapes
    //         .iter()
    //         .filter(|rc_shape| Rc::strong_count(rc_shape) > 1)
    //         .cloned()
    //         .collect();
    // }
}
