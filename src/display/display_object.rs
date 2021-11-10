use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::Geom;
use crate::shaders::ShaderConstant;
use web_sys::WebGl2RenderingContext;
use gl_matrix::mat3;
use super::attribs::Attribs;

pub struct DisplayObject<'a> {
    ctx: WebGl2RenderingContext,
    geom: &'a Geom,
    attribs: Attribs,
}

impl DisplayObject<'_> {
    pub fn new<'a>(ctx: &WebGl2RenderingContext, geom: &'a Geom) -> DisplayObject<'a> {
        let ctx = ctx.clone();
        let attribs = Attribs::new(&ctx, geom);

        DisplayObject { ctx, geom, attribs }
    }
    pub fn draw(&self) {
        let gl_program = ShaderProgram::new(&self.ctx);

        // TODO: Calculate vertices, transformation mat
        self.attribs.set_attributes(&gl_program);
        self.set_u_matrix(&gl_program);
        // TODO: Set uniforms
        self.ctx
            .draw_arrays(self.geom.mode, 0, self.geom.vertex_count);
    }

    pub fn set_u_matrix(&self, program: &ShaderProgram) {
        let matrix_loc = program.get_uniform_loc(ShaderConstant::UProjectionMatrix.to_string());
        let proj_mat = mat3::create();

        self.ctx.uniform_matrix3fv_with_f32_array(
                matrix_loc,
                false,
                &proj_mat
            )
    }
}
