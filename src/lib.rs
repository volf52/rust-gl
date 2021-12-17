use crate::core::application::{Application, CanvasDimensions};
use animation::slide::{self, slide_anim, Axis};
use graphics::shapes::shape_enum::ConstructTextured;
use graphics::Container;
use keyframe::functions::{EaseIn, EaseInOut, EaseInOutCubic};
use keyframe::*;
use textures::typer_text::text_typer;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlTexture;

use crate::graphics::shapes::shape_enum::ConstructAtOrigin;
use crate::graphics::shapes::shape_enum::Shape::Polygon;
use crate::graphics::shapes::{Circle, IrregularPolygon, RegularPolygon, Shape, Triangle};

mod animation;
mod core;
mod display;
mod graphics;
mod math;
mod shaders;
mod textures;
mod ticker;
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
pub fn hey(mat: Vec<f32>) {
    console_log!("{:#?}", mat);
}

#[wasm_bindgen]
pub fn hey2(mat: Vec<f32>) {
    console_log!("{:#?}", mat);
}

#[wasm_bindgen]
pub fn puts(str: &str) {
    console_log!("{}", str);
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

    let red: Vec<u8> = vec![255, 0, 0];
    let _green: Vec<u8> = vec![0, 255, 0];
    let _blue: Vec<u8> = vec![0, 0, 255];

    let mut container = Container::default();
    let mut app = Application::new(&context, dims);

    let _tex = app.tex_from_img("../assets/test.jpg");
    let _text: WebGlTexture = app.text_texture("test", "sans-serif", 40, "white", 20.0, 0.0);

    let text = r##"
            <block width="100" x="0" y="30" text-align="center">
                <s font-size="12" color="#FFFFFF" line-height="1.2"><s color="#FF0000" font-size="24">Paprika</s> is an incredibly good movie.</s>
            </block>"##;

    let text_texture = text_typer(&context, text);
    let c = Circle::new(0, 0, 100.0, &text_texture);
    c.translate(-150.0, 75.0);

    let mut p = Polygon(75.0, 5).new_at_origin_textured(text_texture);

    p.translate2(240.0, 0.0);
    //app.add_shape(&c);
    app.add_shape(&p);
    let tr = Triangle::new_at_origin(100.0, &red);
    //container.add_shape(&tr);
    app.add_container(&container);
    //   app.render();

    let mut sequence = keyframes![(5.0, 0.0, EaseInOut), (0.0, 1.0)];
    app.run(move || {
        slide_anim(&mut p, -300, 200, &mut sequence, Axis::Xneg);
        tr.rotate_deg(5.0);
    });

    Ok(())
}

fn example(time: f32) -> f32 {
    let a = 5.0;
    let b = 0.0;

    ease_with_scaled_time(EaseInOutCubic, a, b, time, 3.0)
}
