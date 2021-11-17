use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use utils::{console_error, console_log};

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{Rectangle, Shape, Triangle};

mod core;
mod display;
mod graphics;
mod math;
mod shaders;
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

    let rectangle = Rectangle::new(0.9, 0.6);
    let triangle = Triangle::new(0.4);

    let mut app = Application::new(&context, dims);

    app.add_shape(&rectangle);
    app.add_shape(&triangle);

    rectangle.rotate(0.7);
    triangle.rotate(0.3);

    // TODO: simulate timeout

    app.render_all();

    Ok(())
}
