mod core;
mod display;
mod graphics;
mod math;
mod shaders;
mod utils;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::geom::polygon::RegularPolygon;
use crate::graphics::geom::triangle::Triangle;
use graphics::geom::polygon::IrregularPolygon;
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

    let application = Application::new(&context, dims);
    let red = vec![1.0, 0.0, 0.0];

    let polygon2 = RegularPolygon {
        radius: 70.0,
        sides: 7,
        color: red.clone(),
    };

    let triangle = Triangle::new(150.0, red.clone());

    let poly = IrregularPolygon::new_from_path(
        vec![0.0, 0.0, 200.0, 200.0, 300.0, 100.0, -100.0, 100.0],
        &red.clone(),
    );


    application.draw_from_origin(&triangle.rotate_deg(45.0));
    application.draw_shape(&poly.rotate_deg(-15.0), 200.0, 100.0);

    Ok(())
}

