use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use utils::{console_error, console_log};

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{
    Circle, Ellipse, IrregularPolygon, Rectangle, RegularPolygon, Shape, Triangle,
};

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
    let green: Vec<f32> = vec![0.0, 1.0, 0.0];
    let blue: Vec<f32> = vec![0.0, 0.0, 1.0];

    let irregular_p = IrregularPolygon::new(300.0, 250.0, 4, &blue);
    let polygon = RegularPolygon::new(200.0, 7, &red);

    let ellipse = Ellipse::new(150.0, 100.0, &blue);
    let circle = Circle::new(120.0, &green);
    let rectangle = Rectangle::new(100.0, 75.0);
    let triangle = Triangle::new(60.0);

    let mut app = Application::new(&context, dims);

    app.add_shape(&irregular_p);
    app.add_shape(&polygon);
    app.add_shape(&ellipse);
    app.add_shape(&circle);
    app.add_shape(&rectangle);
    app.add_shape(&triangle);

    ellipse.rotate(0.4);
    rectangle.rotate(0.7);
    triangle.rotate(0.3);
    irregular_p.rotate(-0.3);

    // TODO: simulate timeout

    app.render_all();

    Ok(())
}
