#![allow(dead_code, unused_variables, unused_imports, unused_macros)]
use std::{cell::RefCell, rc::Rc};

use animation::slide::slide;
use animation::slide::Axis;
use graphics::scene_graph::GraphEntity;
use graphics::Container;
use keyframe::functions::EaseInOut;
use keyframe::keyframes;
use keyframe::AnimationSequence;
use math::bounds::Bounded;
use math::Matrix;
use textures::ab_text::test_tex2;
use textures::texture_text::test_tex;
use textures::typer_text::text_typer;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlTexture;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{
    Circle, Ellipse, IrregularPolygon, Rectangle, RegularPolygon, Shape, Triangle,
};
// use webgl2_glyph_atlas::Renderer;
mod animation;
mod core;
mod display;
mod graphics;
mod math;
mod shaders;
mod textures;
mod utils;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn test_error() {
    console_error!("testing console.error");
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let dims = CanvasDimensions {
        width: canvas.client_width() as f32,
        height: canvas.client_height() as f32,
    };

    let mut app = Application::new(&context, dims);
    let mut container = Container::default();
    app.add_container(&container);

    let red: Vec<u8> = vec![255, 0, 0];
    let green: Vec<u8> = vec![180, 180, 180];
    let blue: Vec<u8> = vec![0, 0, 255];

    let _tex = app.tex_from_img("../assets/test.jpg");
    let _tex: WebGlTexture = app.text_texture("test", "sans-serif", 40, "white", 20.0, 0.0);

    let text = r##"<block width="100" x="0" y="30" text-align="center">
                <s font-size="12" color="#FFFFFF" line-height="1.2"><s color="#FF0000" font-size="24">Paprika</s> is an incredibly good movie.</s>
            </block>"##;

    let tex = text_typer(&context, text);

    let c = Circle::new_at(0.0, -80.0, 100.0, &tex);

    container.add_shape(&c);

    let app_ref = Rc::new(app);
    let shape_ref = Rc::new(RefCell::new(c));

    slide(
        shape_ref.clone(),
        -180,
        180,
        2,
        EaseInOut,
        Axis::Y,
        app_ref.clone(),
    );

    Ok(())
}
