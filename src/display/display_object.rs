use crate::display::shader_program::ShaderProgram;
use crate::graphics::geom::Geom;
use web_sys::WebGl2RenderingContext;

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
        // TODO: Set uniforms

        self.ctx
            .draw_arrays(self.geom.mode, 0, self.geom.vertex_count);
    }
}
