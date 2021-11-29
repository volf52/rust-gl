use graphics::Container;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{Circle, IrregularPolygon, Rectangle, Shape, Triangle};

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

    let _red: Vec<f32> = vec![1.0, 0.0, 0.0];
    let _green: Vec<f32> = vec![0.0, 1.0, 0.0];
    let blue: Vec<f32> = vec![0.0, 0.0, 1.0];

    // let irregular_p = IrregularPolygon::new_at_origin(300.0, 250.0, 4, &blue);
    // let polygon = RegularPolygon::new_at_origin(200.0, 7, &red);

    // let ellipse = Ellipse::new_at_origin(150.0, 100.0, &blue);
    let _circle = Circle::new_at_origin(120.0, &blue);
    let rectangle = Rectangle::new_at_origin(100.0, 75.0);
    // let triangle = Triangle::new(100, 100, 60.0);
    let triangle = Triangle::new_at_origin(75.0);
    triangle.rotate_deg(5.0); // total 45.0 deg rotation for triangle

    let mut container = Container::default();
    container.rotate_deg(55.0); // total 90 deg rotation for rectangle

    // container.add_shape(&circle);
    container.add_shape(&rectangle);

    let poly = IrregularPolygon::new_from_path(
        vec![0.0, 0.0, 200.0, 200.0, 300.0, 100.0, -100.0, 100.0],
        &blue,
    );

    let mut app = Application::new(&context, dims);

    app.add_shape(&poly);
    poly.translate(-70.0, 0.0);

    // app.stage.rotate_deg(35.0);
    triangle.rotate_deg(5.0);

    app.add_container(&container);

    app.add_shape(&triangle);

    app.render();

    Ok(())
}
