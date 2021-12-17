use std::cell::RefCell;
use std::io::Seek;
use std::rc::Rc;

use crate::graphics::shapes::IrregularPolygon;
use crate::textures::texture_img::load_texture_image;
use crate::textures::texture_text::create_text_texture;
use crate::textures::utils::{TextureGen, TextureOrColor};
use crate::ticker::ticker::Ticker;
use keyframe::functions::EaseInOut;
use keyframe::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::display::display_object::DisplayObject;
use crate::graphics::{Container, Shape};
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
    pub ctx: WebGl2RenderingContext,
    pub proj_mat: Matrix,
    pub stage: Container,
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

        let proj_mat = Matrix::projection(dims.width, dims.height);

        Application {
            ctx,
            proj_mat,
            stage: Container::default(),
        }
    }

    pub fn render(&self) {
        self.ctx.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        self.stage.render(self, &Matrix::new());
    }

    pub fn add_container(&mut self, container: &Container) {
        self.stage.add_container(container);
    }

    pub fn add_shape(&mut self, shape: &impl Shape) {
        self.stage.add_shape(shape);
    }

    pub fn draw_textured_shape<T: Shape>(&self, shape: &T, texture: &WebGlTexture) {
        DisplayObject::new(&self.ctx, shape.get_geom()).draw_textured(&self.proj_mat, &texture);
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
        load_texture_image(&self.ctx, src)
    }

    pub fn text_texture(
        &self,
        text: &str,
        font: &str,
        text_size: u32,
        color: &str,
        tx: f32,
        ty: f32,
    ) -> WebGlTexture {
        create_text_texture(&self.ctx, text, font, text_size, color, tx, ty)
    }

    pub fn run<F>(self, closure: F)
    where
        F: 'static + FnMut(),
    {
        render_loop(closure, self);
    }

    pub fn fade_in(
        &self,
        shape: IrregularPolygon,
        from: i32,
        to: i32,
        step: u32,
        duration: u32,
        fps: u32,
    ) {
        shape.translate((shape.x - from) as f32, 0.0);
        let translations: Vec<f32> = (0..duration * fps)
            .map(|f| {
                ease_with_scaled_time(EaseInOut, from as f32, to as f32, f as f32, duration as f32)
            })
            .collect();

        translations.iter().for_each(|f| {
            shape.translate(f.clone(), 0.0);
        })
    }

    // pub fn run2(&self, ticker: &Ticker) {
    //     render_loop2(Ticker::on_draw(self), || println!("hello"));
    // }

    // pub fn gc(&mut self) {
    //     self.shapes = self
    //         .shapes
    //         .iter()
    //         .filter(|rc_shape| Rc::strong_count(rc_shape) > 1)
    //         .cloned(k
    //         .collect();
    // }
}

impl Ticker {
    pub fn on_draw(app: &Application) {
        app.render();
    }
}
pub fn render_loop<F>(mut closure: F, app: Application)
where
    F: 'static + FnMut(),
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        app.render();
        closure();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub fn render_loop2<F>(mut closure: F)
where
    F: 'static + FnMut(),
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        closure();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
