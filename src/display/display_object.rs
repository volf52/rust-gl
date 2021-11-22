use super::attribs::Attribs;
use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::geom::Geom;
use crate::math::Matrix;
use crate::shaders::ShaderConstant;
use web_sys::WebGl2RenderingContext;

pub struct DisplayObject<'a> {
    ctx: WebGl2RenderingContext,
    geom: &'a Geom,
    attribs: Attribs,
    proj_mat: Matrix,
}

impl DisplayObject<'_> {
    pub fn new<'a>(
        ctx: &WebGl2RenderingContext,
        geom: &'a Geom,
        proj_mat: Matrix,
    ) -> DisplayObject<'a> {
        let ctx = ctx.clone();
        let attribs = Attribs::new(&ctx, geom);

        DisplayObject {
            ctx,
            geom,
            attribs,
            proj_mat,
        }
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

        self.ctx
            .uniform_matrix3fv_with_f32_array(matrix_loc, false, &self.proj_mat.to_array())
    }

}
