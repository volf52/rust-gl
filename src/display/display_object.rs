use crate::display::buffers::BufferInfo;
use crate::graphics::shape::Shape;
use web_sys::{WebGl2RenderingContext};
use crate::display::shader_program::ShaderProgram;

/**
#[derive(Debug, Clone)]
pub struct DisplayObject<'a, T: Shape + Clone> {
    ctx: &'a WebGl2RenderingContext,
    shape: &'a T,
}

impl<'a, T> DisplayObject<'a, T>
    where
        T: Shape + Clone
{
    pub fn new(ctx: &'a WebGl2RenderingContext, shape: &'a T) -> Self {
        DisplayObject {
            ctx,
            shape,
        }
    }
    pub fn draw(&self) {
        let gl_program = ShaderProgram::new(&self.ctx);
        let buffer_info: BufferInfo = BufferInfo::new(&self.ctx, self.shape);
        buffer_info.set_attributes_and_buffers(&gl_program);
        self.ctx
            .draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, self.shape.vertex_count());
    }
}
 **/

pub struct DisplayObject {
    ctx: WebGl2RenderingContext,
    shape: Box<dyn Shape>,
}

impl DisplayObject
{
    pub fn new(ctx: &WebGl2RenderingContext, shape: Box<dyn Shape>) -> DisplayObject {
        let ctx = ctx.clone();
        DisplayObject {
            ctx,
            shape,
        }
    }
    pub fn draw(&self) {
        let gl_program = ShaderProgram::new(&self.ctx);
        let buffer_info: BufferInfo = BufferInfo::new(&self.ctx, &*self.shape);
        buffer_info.set_attributes_and_buffers(&gl_program);
        self.ctx
            .draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, self.shape.vertex_count());
    }
}

