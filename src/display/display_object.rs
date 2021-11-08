use crate::display::shader_program::ShaderProgram;
use crate::graphics::shape::Shape;
use web_sys::WebGl2RenderingContext;

use super::attribs::Attribs;

// #[derive(Debug, Clone)]
// pub struct DisplayObject<'a, T: Shape + Clone> {
//     ctx: &'a WebGl2RenderingContext,
//     shape: &'a T,
// }

// impl<'a, T> DisplayObject<'a, T>
// where
//     T: Shape + Clone,
// {
//     pub fn new(ctx: &'a WebGl2RenderingContext, shape: &'a T) -> Self {
//         DisplayObject { ctx, shape }
//     }
//     pub fn draw(&self) {
//         let gl_program = ShaderProgram::new(&self.ctx);
//         let buffer_info: BufferInfo = BufferInfo::new(&self.ctx, self.shape);
//         buffer_info.set_attributes_and_buffers(&gl_program);
//         self.ctx.draw_arrays(
//             WebGl2RenderingContext::TRIANGLE_STRIP,
//             0,
//             self.shape.vertex_count(),
//         );
//     }
// }

pub struct DisplayObject {
    ctx: WebGl2RenderingContext,
    shape: Box<dyn Shape>,
    attribs: Attribs,
}

impl DisplayObject {
    pub fn new(ctx: &WebGl2RenderingContext, shape: Box<dyn Shape>) -> DisplayObject {
        let ctx = ctx.clone();
        let attribs = Attribs::new(&ctx, &*shape);

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

        let mode = self.shape.mode();
        let vert_count = self.shape.vertex_count();

        self.ctx.draw_arrays(mode, 0, vert_count);
    }
}
