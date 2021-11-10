use crate::display::display_object::DisplayObject;
use crate::graphics::shape::Shape;
use crate::math::Matrix;
use web_sys::WebGl2RenderingContext;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Application {
    ctx: WebGl2RenderingContext,
    shapes: Vec<Shape>,
    dims: CanvasDimensions
}

pub struct CanvasDimensions {
    pub width: f32,
    pub height: f32
}
impl Application {
    pub fn new(ctx: &WebGl2RenderingContext, shapes: Vec<Shape>, dims: CanvasDimensions) -> Self {
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
        Application { ctx, shapes, dims }
    }
    pub fn add_shape(&mut self, g: Shape) {
        self.shapes.push(g);
    }
    pub fn render_all(&self) {
        // TODO: calculate base projection mat
        let geoms = self.shapes.iter().map(|f| f.new());
        geoms.for_each(|f| {
            let mat = Matrix::rotate(&Matrix::new(), 0.4);
            DisplayObject::new(&self.ctx, &f, mat).draw();
        })
    }
}
