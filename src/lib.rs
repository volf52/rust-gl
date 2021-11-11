mod core;
mod display;
mod graphics;
mod math;
mod shaders;
mod utils;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::{quad::Rectangle, shape::Shape, triangle::Triangle};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

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
pub fn hey() {
    console_log!("testing console log");
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

    let rectangle = Rc::new(RefCell::new(Rectangle::new(0.9, 0.6)));
    // let square = Shape::Square { size: 0.5 };

    let mut app = Application::new(&context, dims);

    app.add_shape(rectangle.clone());

    {
        let triangle = Rc::new(RefCell::new(Triangle::new(0.6)));
        app.add_shape(triangle.clone());
        triangle.borrow_mut().rotate(0.1);
    }

    app.render_all();

    // app.render_all();
    // TODO: simulate timeout

    rectangle.borrow_mut().rotate(0.4);

    Ok(())
}
