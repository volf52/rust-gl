use web_sys::{WebGl2RenderingContext, WebGlShader};
pub trait Shader {
    fn compile(ctx: &WebGl2RenderingContext) -> (WebGlShader, WebGlShader);
}

macro_rules! DEFAULT_VS {
    () => {
        "attribute vec2 {a_position};
         attribute vec2 {a_texture_coord};

         uniform mat3 {u_projection};
         uniform mat3 {u_model};

         varying vec2 {v_texture_coord};

         void main(void) {{
            gl_Position = vec4(({u_projection} * {u_model} * vec3({a_position}, 1.0)).xy, 0.0, 1.0);
            {v_texture_coord} = {a_texture_coord};
        }}"
    };
}

macro_rules! DEFAULT_FS {
    () => {
        "
        varying highp vec2 {v_texture_coord};
        uniform sampler2D {u_sampler};

        void main(void){{
           gl_FragColor = texture2D({u_sampler}, {v_texture_coord});
        }}"
    };
}

pub(crate) use DEFAULT_FS;
pub(crate) use DEFAULT_VS;

pub fn compile_shaders(
    ctx: &WebGl2RenderingContext,
    vs_src: &str,
    fs_src: &str,
) -> (WebGlShader, WebGlShader) {
    let vert_shader = compile_shader(ctx, WebGl2RenderingContext::VERTEX_SHADER, vs_src).unwrap();
    let frag_shader = compile_shader(ctx, WebGl2RenderingContext::FRAGMENT_SHADER, fs_src).unwrap();

    (vert_shader, frag_shader)
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader Object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
