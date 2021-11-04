use crate::attribs::Attribs;
use crate::gl::GlProgram;
use crate::gl_utils::{bind_u8_buffer_data, create_buffer_with_type};
use crate::square::Square;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct BufferInfo {
    attribs: Attribs,
    indices: WebGlBuffer,
}

#[wasm_bindgen]
impl BufferInfo {
    pub fn create_buffer_info(ctx: &WebGl2RenderingContext, s: &Square) -> Self {
        let attribs = Attribs::new(ctx, s);

        let indices = create_buffer_with_type(ctx, WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER);
        bind_u8_buffer_data(ctx, &s.indices());

        BufferInfo { attribs, indices }
    }

    pub fn set_attributes_and_buffers(&self, prog: &GlProgram) {
        self.attribs.set_attributes(prog);
        let ctx = prog.context();

        ctx.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.indices),
        );
    }
}
