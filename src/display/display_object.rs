use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::Geom;
use crate::math::Matrix;
use crate::shaders::ShaderConstant;

use super::attribs::Attribs;

pub struct DisplayObject {
    ctx: WebGl2RenderingContext,
    geom: Rc<RefCell<Geom>>,
    attribs: Attribs,
}

impl DisplayObject {
    pub fn new(ctx: &WebGl2RenderingContext, geom: Rc<RefCell<Geom>>) -> DisplayObject {
        let ctx = ctx.clone();
        let attribs = Attribs::new(&ctx, geom.borrow());

        DisplayObject { ctx, geom, attribs }
    }
    pub fn draw(&self) {
        let gl_program = ShaderProgram::new(&self.ctx);

        self.attribs.set_attributes(&gl_program);
        let geom = self.geom.borrow();
        self.set_u_matrix(&gl_program, &geom.u_mat);
        self.ctx.draw_arrays(geom.mode, 0, geom.vertex_count);
    }

    pub fn set_u_matrix(&self, program: &ShaderProgram, mat: &Matrix) {
        let matrix_loc = program.get_uniform_loc(ShaderConstant::UProjectionMatrix.to_string());

        self.ctx
            .uniform_matrix3fv_with_f32_array(matrix_loc, false, &mat.to_array())
    }
}
