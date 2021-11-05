use self::ShaderConstant::*;
use std::fmt::{self, Display};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlShader};
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderConstant {
    APosition,
    AColor,
    UProjectionMatrix,
    VColor,
    USampler,
}

pub const ATTRIBUTES: [ShaderConstant; 2] = [APosition, AColor];
// pub const UNIFORMS: [ShaderConstant; 2] = [UProjectionMatrix, USampler];
pub const UNIFORMS: [ShaderConstant; 0] = [];

impl Display for ShaderConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn compile_shaders(ctx: &WebGl2RenderingContext) -> (WebGlShader, WebGlShader) {
    let vertex_shader_src = format!(
        "attribute vec2 {a_postition};
        attribute vec3 {a_color};

        varying vec3 {v_color};

        void main(void) {{
            gl_Position = vec4({a_postition}, 0.0, 1.0);
            {v_color} = {a_color};
        }}
        ",
        a_postition = ShaderConstant::APosition.to_string(),
        a_color = ShaderConstant::AColor.to_string(),
        v_color = ShaderConstant::VColor.to_string(),
    );

    let vert_shader = compile_shader(
        ctx,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vertex_shader_src,
    )
    .unwrap();

    let frag_shader_src = format!(
        "precision mediump float;
        varying vec3 {v_color};

        void main(void) {{
            gl_FragColor = vec4({v_color}, 1.0);
            // gl_FragColor = vec4(1.0, 0.0, 0.4, 1.0);
        }}
        ",
        v_color = ShaderConstant::VColor.to_string(),
    );

    let frag_shader = compile_shader(
        ctx,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &frag_shader_src,
    )
    .unwrap();

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
