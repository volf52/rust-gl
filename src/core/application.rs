use crate::display::display_object::DisplayObject;
use crate::graphics::geom::geom::Geom;
use crate::math::Matrix;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
pub struct Application {
    ctx: WebGl2RenderingContext,
    dims: CanvasDimensions,
}
pub struct CanvasDimensions {
    pub width: f32,
    pub height: f32,
}

impl Application {
    pub fn new(ctx: &WebGl2RenderingContext, dims: CanvasDimensions) -> Self {
        let ctx = ctx.clone();
        //initialize context need to happens once
        ctx.viewport(
            0,
            0,
            ctx.drawing_buffer_width(),
            ctx.drawing_buffer_height(),
        );
        ctx.clear_color(0.8, 0.9, 1.0, 1.0);
        ctx.clear_depth(1.0);
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST);
        ctx.depth_func(WebGl2RenderingContext::LEQUAL);
        ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        Application { ctx, dims }
    }

    pub fn draw_shape(&self, geom: &Geom, x: f32 , y: f32) {
        let proj_mat =
            geom.matrix.translate(&(-self.dims.width / 2.0), &(-self.dims.height / 2.0));

        DisplayObject::new(
            &self.ctx,
            &geom,
            proj_mat
                .translate(&x, &y)
                .project(&self.dims.width, &self.dims.height),
        )
        .draw();
    }

    pub fn draw_from_origin(&self, geom: &Geom) {
        DisplayObject::new(
            &self.ctx,
            &geom,
            geom.matrix.project(&self.dims.width, &self.dims.height),
        )
        .draw();
    }

    pub fn projection(&self, mat: &Matrix) -> Matrix {
        mat.project(&self.dims.width, &self.dims.height)
    }
}
