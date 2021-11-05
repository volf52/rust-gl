use crate::attribs::Attribs;
use crate::gl_program::GlProgram;
use crate::square::Square;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct BufferInfo {
    attribs: Attribs,
}

#[wasm_bindgen]
impl BufferInfo {
    pub fn create_buffer_info(ctx: &WebGl2RenderingContext, s: &Square) -> Self {
        let attribs = Attribs::new(ctx, s);

        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        BufferInfo { attribs }
    }

    pub fn set_attributes_and_buffers(&self, prog: &GlProgram) {
        self.attribs.set_attributes(prog);
    }
}
