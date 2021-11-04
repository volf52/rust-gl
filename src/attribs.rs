use crate::gl::GlProgram;
use crate::gl_utils::{bind_f32_buffer_data, bind_u8_buffer_data, create_array_buffer};
use crate::square::Square;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Attrib {
    num_components: usize,
    type_: u32,
    normalize: bool,
    buffer: WebGlBuffer,
}

impl Attrib {
    pub fn from_f32(ctx: &WebGl2RenderingContext, data: &Vec<f32>) -> Self {
        let num_components = data.len();
        let buffer = create_array_buffer(ctx);

        bind_f32_buffer_data(ctx, data);

        Attrib {
            num_components,
            buffer,
            type_: WebGl2RenderingContext::FLOAT,
            normalize: false,
        }
    }

    pub fn from_u8(ctx: &WebGl2RenderingContext, data: &Vec<u8>) -> Self {
        let num_components = data.len();
        let buffer = create_array_buffer(ctx);

        bind_u8_buffer_data(ctx, data);

        Attrib {
            num_components,
            buffer,
            type_: WebGl2RenderingContext::UNSIGNED_BYTE,
            normalize: true,
        }
    }

    pub fn set_attribute() {}
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Attribs {
    position: Attrib,
    tex_coord: Attrib,
    normal: Attrib,
    color: Attrib,
}

impl Attribs {
    pub fn new(ctx: &WebGl2RenderingContext, s: &Square) -> Self {
        let position = Attrib::from_f32(ctx, &s.position());
        let tex_coord = Attrib::from_f32(ctx, &s.texcoord());
        let normal = Attrib::from_f32(ctx, &s.normal());
        let color = Attrib::from_u8(ctx, &s.color());

        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Attribs {
            position,
            tex_coord,
            normal,
            color,
        }
    }

    // TODO: Update set attributes
    pub fn set_attributes(&self, program: &GlProgram) {}
}
