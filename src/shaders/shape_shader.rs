use crate::shaders::shader_utils::compile_shaders;

use super::shader_utils::{self, Shader};
use super::ShaderConstant;
use web_sys::{WebGl2RenderingContext, WebGlShader};

pub struct ShapeShader {}

impl ShapeShader {
    pub fn new(ctx: &WebGl2RenderingContext) -> (WebGlShader, WebGlShader) {
        return ShapeShader::compile(ctx);
    }
}

impl Shader for ShapeShader {
    fn compile(ctx: &WebGl2RenderingContext) -> (WebGlShader, WebGlShader) {
        let vs_src = format!(
            shader_utils::DEFAULT_VS!(),
            a_position = ShaderConstant::APosition.to_string(),
            a_color = ShaderConstant::AColor.to_string(),
            v_color = ShaderConstant::VColor.to_string(),
            u_model = ShaderConstant::UModel.to_string()
        );
        let fs_src = format!(
            shader_utils::DEFAULT_FS!(),
            v_color = ShaderConstant::VColor.to_string()
        );

        compile_shaders(ctx, vs_src.as_str(), fs_src.as_str())
    }
}
