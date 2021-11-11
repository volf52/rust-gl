use crate::display::display_object::DisplayObject;
use crate::graphics::shape::Shape;
use crate::math::Matrix;
use std::cell::RefCell;
use web_sys::WebGl2RenderingContext;

pub struct Application<'a> {
    ctx: WebGl2RenderingContext,
    shapes: Vec<&'a RefCell<dyn Shape>>,
    dims: CanvasDimensions,
}

pub struct CanvasDimensions {
    pub width: f32,
    pub height: f32,
}

impl<'a> Application<'a> {
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
    pub fn add_shape(&mut self, g: &'a RefCell<dyn Shape>) {
        self.shapes.push(g);
    }
    pub fn render_all(&self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        let mat = &Matrix::new();
        self.shapes.iter().for_each(|rc_shape| {
            let geom = rc_shape.borrow().get_geom();
            DisplayObject::new(&self.ctx, &geom, mat).draw();
        });
    }
}
