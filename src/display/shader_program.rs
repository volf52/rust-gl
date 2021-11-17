use crate::shaders::{shape_shader::ShapeShader, ShaderConstant, ATTRIBUTES, UNIFORMS};
use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[derive(Clone, Debug)]
pub struct ShaderProgram {
    ctx: WebGl2RenderingContext,
    prog: WebGlProgram,
    a_locations: HashMap<String, i32>,
    u_locations: HashMap<String, WebGlUniformLocation>,
}

impl ShaderProgram {
    pub fn new(ctx: &WebGl2RenderingContext) -> Self {
        let ctx = ctx.clone();
        let (vert_shader, frag_shader) = ShapeShader::new(&ctx);
        let prog = link_program(&ctx, &vert_shader, &frag_shader).unwrap();
        let a_locations = HashMap::new();
        let u_locations = HashMap::new();
        let mut gl_prog = ShaderProgram {
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

    pub fn get_uniform_loc(&self, s: String) -> Option<&WebGlUniformLocation> {
        match self.u_locations.get(&s) {
            Some(x) => Some(x),
            None => None,
        }
    }
}

impl ShaderProgram {
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
