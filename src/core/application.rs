use crate::animation::another_anim::SlideAnim;
use crate::textures::{texture_img::load_texture_image, texture_text::create_text_texture};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Performance, WebGl2RenderingContext, WebGlTexture, Window};

use crate::graphics::{Container, Shape};
use crate::math::Matrix;

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
    pub curr_time: f64,
    pub queued_anims: Vec<SlideAnim>,
    perf: Performance,
}
pub struct CanvasDimensions {
    pub width: f32,
    pub height: f32,
}

impl Application {
    pub fn new(ctx: &WebGl2RenderingContext, window: &Window, dims: CanvasDimensions) -> Self {
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

        // load textures in the correct direction: https://jameshfisher.com/2020/10/22/why-is-my-webgl-texture-upside-down/
        ctx.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);

        let proj_mat = Matrix::projection(dims.width, dims.height);
        let perf = window
            .performance()
            .expect("Performance should be available");

        Application {
            ctx,
            proj_mat,
            stage: Container::default(),
            queued_anims: Vec::new(),
            perf,
            curr_time: 0.0,
        }
    }

    pub fn update_curr_frame_time(&mut self) -> f64 {
        let curr_time = self.perf.now() / 1_000.0;
        let elapsed = curr_time - self.curr_time;
        self.curr_time = curr_time;

        elapsed
    }

    pub fn advance_anims(&mut self) {
        let dt = self.update_curr_frame_time();

        // filter out animations that are done
        for anim in self.queued_anims.iter_mut() {
            anim.tick(dt);
        }
    }

    pub fn enqueue_anim(&mut self, anim: SlideAnim) {
        self.queued_anims.push(anim);
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

    // pub fn gc(&mut self) {
    //     self.shapes = self
    //         .shapes
    //         .iter()
    //         .filter(|rc_shape| Rc::strong_count(rc_shape) > 1)
    //         .cloned(k
    //         .collect();
    // }
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
