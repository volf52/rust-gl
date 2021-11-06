use web_sys::WebGl2RenderingContext;
use crate::display::attribs::Attribs;
use crate::display::shader_program::ShaderProgram;
use crate::graphics::shape::Shape;

pub struct BufferInfo {
    attribs: Attribs,
}

impl BufferInfo {
    pub fn new(ctx: &WebGl2RenderingContext, s: &dyn Shape) -> Self {
        let attribs = Attribs::new(ctx, s);
        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        BufferInfo { attribs }
    }

    pub fn set_attributes_and_buffers(&self, prog: &ShaderProgram) {
        self.attribs.set_attributes(prog);
    }
}
