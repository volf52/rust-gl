use crate::buffers::BufferInfo;
use crate::square::Square;
use std::collections::HashMap;

use crate::shaders::{compile_shaders, ShaderConstant, ATTRIBUTES, UNIFORMS};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GlProgram {
    ctx: WebGl2RenderingContext,
    prog: WebGlProgram,
    a_locations: HashMap<String, i32>,
    u_locations: HashMap<String, WebGlUniformLocation>,
}

#[wasm_bindgen]
impl GlProgram {
    pub fn new(ctx: &WebGl2RenderingContext, width: i32, height: i32) -> Self {
        let ctx = ctx.clone();
        ctx.viewport(0, 0, width, height);

        let (vert_shader, frag_shader) = compile_shaders(&ctx);

        let prog = link_program(&ctx, &vert_shader, &frag_shader).unwrap();

        let a_locations = HashMap::new();
        let u_locations = HashMap::new();

        let mut gl_prog = GlProgram {
            ctx,
            prog,
            a_locations,
            u_locations,
        };
        gl_prog.use_program();
        gl_prog.insert_a_locations();
        gl_prog.insert_u_locations();

        gl_prog
    }

    pub fn draw(&self, s: &Square, buffer_info: &BufferInfo) {
        self.ctx.viewport(
            0,
            0,
            self.ctx.drawing_buffer_width(),
            self.ctx.drawing_buffer_height(),
        );
        self.ctx.clear_color(0.8, 0.9, 1.0, 1.0);
        self.ctx.clear_depth(1.0);
        self.ctx.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.ctx.depth_func(WebGl2RenderingContext::LEQUAL);
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        buffer_info.set_attributes_and_buffers(self);
        // buffer_info set uniforms

        self.ctx
            .draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, s.vert_count);
    }

    pub fn context(&self) -> WebGl2RenderingContext {
        self.ctx.clone()
    }

    pub fn use_program(&self) {
        self.ctx.use_program(Some(&self.prog));
    }

    pub fn get_attrib_loc(&self, s: String) -> Option<i32> {
        match self.a_locations.get(&s) {
            Some(x) => Some(*x),
            None => None,
        }
    }

    pub fn get_uniform_loc(&self, s: String) -> Option<WebGlUniformLocation> {
        match self.u_locations.get(&s) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}

impl GlProgram {
    fn insert_a_locations(&mut self) {
        for c in ATTRIBUTES {
            let loc = self.get_attrib_location(&c);
            self.a_locations.insert(c.to_string(), loc);
        }
    }

    fn insert_u_locations(&mut self) {
        for c in UNIFORMS {
            let loc = self.get_uniform_location(&c);
            match loc {
                Some(x) => self.u_locations.insert(c.to_string(), x),
                _ => None,
            };
        }
    }

    pub fn get_attrib_location(&self, attrib: &ShaderConstant) -> i32 {
        self.ctx
            .get_attrib_location(&self.prog, &attrib.to_string())
    }

    pub fn get_uniform_location(&self, uniform: &ShaderConstant) -> Option<WebGlUniformLocation> {
        self.ctx
            .get_uniform_location(&self.prog, &uniform.to_string())
    }

    pub fn get_a_locations(&self) -> HashMap<String, i32> {
        self.a_locations.clone()
    }

    pub fn get_u_locations(&self) -> HashMap<String, WebGlUniformLocation> {
        self.u_locations.clone()
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create GL program"))
        .unwrap();

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program")))
    }
}
