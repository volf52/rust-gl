use crate::display::shader_program::ShaderProgram;
use crate::graphics::shape::Shape;
use web_sys::WebGl2RenderingContext;

use super::attribs::Attribs;

pub struct DisplayObject<'a> {
    ctx: WebGl2RenderingContext,
    shape: &'a Shape,
    attribs: Attribs,
}

impl DisplayObject<'_> {
    pub fn new<'a>(ctx: &WebGl2RenderingContext, shape: &'a Shape) -> DisplayObject<'a> {
        let ctx = ctx.clone();
        let attribs = Attribs::new(&ctx, shape);

        DisplayObject {
            ctx,
            shape,
            attribs,
        }
    }
    pub fn draw(&self) {
        let gl_program = ShaderProgram::new(&self.ctx);

        // TODO: Calculate vertices, transformation mat
        self.attribs.set_attributes(&gl_program);
        // TODO: Set uniforms

        self.ctx
            .draw_arrays(self.shape.mode, 0, self.shape.vertex_count);
    }
}
