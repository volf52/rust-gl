use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::display::display_object::DisplayObject;
use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
use crate::textures::solid_texture::create_solid_texture;

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
    shapes: Vec<Rc<RefCell<Geom>>>,
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

    pub fn add_shape(&mut self, shape: &impl Shape) {
        self.shapes.push(shape.get_geom());
    }

    pub fn render_all(&mut self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        // self.gc();

        let proj_mat = Matrix::projection(&self.dims.width, &self.dims.height);

        self.shapes.iter().for_each(|shape_geom| {
            DisplayObject::new(&self.ctx, shape_geom.clone()).draw(&proj_mat);
        });
    }

    pub fn draw_textured_shape(&mut self, shape: &Rc<RefCell<Geom>>, texture: WebGlTexture) {
        let proj_mat = Matrix::projection(&self.dims.width, &self.dims.height);
        DisplayObject::new(&self.ctx, shape.clone()).draw_textured(&proj_mat, texture);
    }

    pub fn draw_colored_shape(&mut self, shape: &Rc<RefCell<Geom>>, color: &Vec<u8>) {
        let proj_mat = Matrix::projection(&self.dims.width, &self.dims.height);
        DisplayObject::new(&self.ctx, shape.clone())
            .draw_textured(&proj_mat, create_solid_texture(&self.ctx, &color));
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
