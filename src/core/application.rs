use crate::load_texture_image;
use crate::textures::utils::{TextureGen, TextureOrColor};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::display::display_object::DisplayObject;
use crate::graphics::Shape;
use crate::math::Matrix;
use crate::textures::solid_texture::create_solid_texture;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct Application {
    ctx: WebGl2RenderingContext,
    // shapes: Vec<Rc<RefCell<Geom>>>,
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
    pub fn clear_all(&self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn draw_textured_shape<T: Shape>(&self, shape: &T, texture: &WebGlTexture) {
        let proj_mat = Matrix::projection(&self.dims.width, &self.dims.height);
        DisplayObject::new(&self.ctx, shape.get_geom()).draw_textured(&proj_mat, &texture);
    }

    pub fn draw_colored_shape<T: Shape>(&self, shape: &T, color: &Vec<u8>) {
        self.draw_textured_shape(shape, &create_solid_texture(&self.ctx, color));
    }

    pub fn draw_shape<T: Shape, U: TextureGen>(&self, shape: &T, mask: &U) {
        let temp = mask.to_enum();
        match temp {
            TextureOrColor::Color(color) => self.draw_colored_shape(shape, &color),
            TextureOrColor::Texture(texture) => self.draw_textured_shape(shape, &texture),
        }
    }

    pub fn tex_from_img(&self, src: &str) -> WebGlTexture {
        load_texture_image(Rc::new(self.ctx.clone()), src)
    }
}
