use std::{cell::RefCell, rc::Rc};

use graphics::Container;
use math::bounding_rect::Bounded;
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

    let _text: WebGlTexture = app.text_texture("test", "sans-serif", 40, "white", 20.0, 0.0);

    let text = r##"
            <block width="100" x="0" y="30" text-align="center">
                <s font-size="12" color="#FFFFFF" line-height="1.2"><s color="#FF0000" font-size="24">Paprika</s> is an incredibly good movie.</s>
            </block>"##;

    let text_texture = text_typer(&context, text);
    let c = Circle::new_at_origin(100.0, &text_texture);

    container.add_shape(&c);

    render_loop(move || {
        app.render();
        // c.rotate_deg(1.0);
    });

    Ok(())
}

pub fn render_loop<F>(mut closure: F)
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
