use crate::display::display_object::DisplayObject;
use crate::graphics::shape::Shape;
use web_sys::WebGl2RenderingContext;

pub struct Application {
    ctx: WebGl2RenderingContext,
    shapes: Vec<Box<dyn Shape>>,
}

impl Application {
    pub fn new(ctx: &WebGl2RenderingContext) -> Self {
        let shapes = Vec::new();
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
        Application { ctx, shapes }
    }
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
    pub fn render_all(&self) {
        // TODO: calculate base projection mat
        for shape in self.shapes.iter().cloned() {
            let display_object = DisplayObject::new(&self.ctx, shape);
            // TODO: pass base projection mat
            display_object.draw();
        }
    }
}
