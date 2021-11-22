use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::geom::Geom;
use crate::shaders::ShaderConstant;
use crate::utils::gl_utils::{bind_f32_buffer_data, create_array_buffer};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Debug, Clone)]
pub struct Attrib {
    num_components: i32,
    buffer: WebGlBuffer,
}

impl Attrib {
    pub fn from_f32(ctx: &WebGl2RenderingContext, data: &Vec<f32>, num_components: i32) -> Self {
        let buffer = create_array_buffer(ctx);

        bind_f32_buffer_data(ctx, data);

        Attrib {
            num_components,
            buffer,
        }
    }

    // pub fn from_u8(ctx: &WebGl2RenderingContext, data: &Vec<u8>, num_components: i32) -> Self {
    //     let buffer = create_array_buffer(ctx);

    //     bind_u8_buffer_data(ctx, data);

    //     Attrib {
    //         num_components,
    //         buffer,
    //     }
    // }
}

impl Attrib {
    pub fn set_attribute(&self, ctx: &WebGl2RenderingContext, loc: u32) {
        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        ctx.enable_vertex_attrib_array(loc);
        ctx.vertex_attrib_pointer_with_i32(
            loc,
            self.num_components,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Attribs {
    position: Attrib,
    color: Attrib,
}

impl Attribs {
    pub fn new(ctx: &WebGl2RenderingContext, g: &Geom) -> Self {
        let position = Attrib::from_f32(ctx, &g.position(), 2);
        let color = Attrib::from_f32(ctx, &g.color(), 3);

        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Attribs { position, color }
    }

    pub fn set_attributes(&self, program: &ShaderProgram) {
        let ctx = program.context();

        let pos_loc = program.get_attrib_loc(ShaderConstant::APosition.to_string());
        match pos_loc {
            Some(x) => self.position.set_attribute(&ctx, x as u32),
            None => (),
        }

        let color_loc = program.get_attrib_loc(ShaderConstant::AColor.to_string());
        match color_loc {
            Some(x) => self.color.set_attribute(&ctx, x as u32),
            None => (),
        }
    }
}
