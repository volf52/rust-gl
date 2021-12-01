use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::Geom;
use crate::math::Matrix;
use crate::shaders::ShaderConstant;
use crate::textures::solid_texture::create_solid_texture;
use crate::textures::utils::TextureOrColor;

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

    pub fn draw(&self, proj_mat: &Matrix, parent_transform_mat: &Matrix) {
        let gl_program = ShaderProgram::new(&self.ctx);
        let geom = self.geom.borrow();

        self.attribs.set_attributes(&gl_program);

        self.set_projection_matrix(&gl_program, proj_mat);

        let texture = match &geom.texture_data {
            TextureOrColor::Color(color) => create_solid_texture(&self.ctx, color),
            TextureOrColor::Texture(t) => t.clone(),
        };

        self.activate_texture(&texture);
        self.set_usampler(&gl_program);

        let model_matrix = Matrix::multiply(parent_transform_mat, &geom.u_mat);
        self.set_model_matrix(&gl_program, &model_matrix);

        self.ctx.draw_arrays(geom.mode, 0, geom.vertex_count);
    }

    pub fn draw_textured(&self, proj_mat: &Matrix, texture: &WebGlTexture) {
        let gl_program = ShaderProgram::new(&self.ctx);
        let geom = self.geom.borrow();

        self.attribs.set_attributes(&gl_program);

        self.set_projection_matrix(&gl_program, proj_mat);
        self.set_model_matrix(&gl_program, &geom.u_mat);
        self.activate_texture(texture);
        self.set_usampler(&gl_program);

        self.ctx.draw_arrays(geom.mode, 0, geom.vertex_count);
    }

    pub fn set_model_matrix(&self, program: &ShaderProgram, mat: &Matrix) {
        DisplayObject::set_matrix(&self.ctx, program, ShaderConstant::UModel, mat);
    }

    pub fn set_projection_matrix(&self, program: &ShaderProgram, mat: &Matrix) {
        DisplayObject::set_matrix(&self.ctx, program, ShaderConstant::UProjection, mat);
    }

    pub fn set_matrix(
        ctx: &WebGl2RenderingContext,
        program: &ShaderProgram,
        type_: ShaderConstant,
        mat: &Matrix,
    ) {
        let matrix_loc = program.get_uniform_loc(type_.to_string());
        ctx.uniform_matrix3fv_with_f32_array(matrix_loc, false, &mat.to_array());
    }

    pub fn set_usampler(&self, program: &ShaderProgram) {
        let loc = program.get_uniform_loc(ShaderConstant::USampler.to_string());
        self.ctx.uniform1i(loc, 0);
    }

    pub fn activate_texture(&self, texture: &WebGlTexture) {
        // self.ctx.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.ctx
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    }
}
