use graphics::container;
use graphics::Container;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{Circle, Rectangle, Shape};

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

    let red: Vec<f32> = vec![1.0, 0.0, 0.0];
    let _green: Vec<f32> = vec![0.0, 1.0, 0.0];
    let blue: Vec<f32> = vec![0.0, 0.0, 1.0];

    let _circle = Circle::new_at_origin(120.0, &blue);
    let rectangle = Rectangle::new_at_origin(300.0, 300.0, &red);
    let rectangle_2 = Rectangle::new(-100, -100, 100.0, 100.0, &blue); // topleft corners of both should line up

    let mut container = Container::default();
    container.move_by(-100.0, -100.0);

    container.add_shape(&rectangle);
    container.add_shape(&rectangle_2);

    let mut app = Application::new(&context, dims);

    app.add_container(&container);

    app.render();

    Ok(())
}
