use graphics::container;
use graphics::Container;
use math::bounding_rect::Bounded;
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

    let c = Circle::new_at(-200, -200, 100.0, &red);

    let mut container = Container::default();
    // container.move_by(-200.0, -200.0);

    c.move_by(200.0, 200.0);

    console_log!("Center for c: {:?}", c.get_center()); // should be 0.0, 0.0

    let p = (-50.0, 50.0);
    console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // true
    console_log!(
        "Contains in bounds {:?} for c: {}",
        p,
        c.contains_in_bounds(p.0, p.1)
    ); // true

    let p = (-90.0, -90.0);
    console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // false
    console_log!(
        "Contains in bounds {:?} for c: {}",
        p,
        c.contains_in_bounds(p.0, p.1)
    ); // true

    let p = (100.1, 100.0);
    console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // false
    console_log!(
        "Contains in bounds {:?} for c: {}",
        p,
        c.contains_in_bounds(p.0, p.1)
    ); // false

    let c_bounds = c.get_bounds();
    let rect = Rectangle::new_at(
        c_bounds.x as i32,
        c_bounds.y as i32,
        c_bounds.width,
        c_bounds.height,
        &blue,
    );

    container.add_shape(&c);

    let mut app = Application::new(&context, dims);

    app.add_shape(&rect);
    app.add_container(&container);

    app.render();

    Ok(())
}
