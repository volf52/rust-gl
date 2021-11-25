use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{IrregularPolygon, RegularPolygon, Shape};
use std::cell::RefCell;
use std::rc::Rc;

use textures::texture_img::load_texture_image;
use textures::texture_img::get_img;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

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
pub fn hey(mat: Vec<f32>) {
    console_log!("{:#?}", mat);
}

#[wasm_bindgen]
pub fn hey2(mat: Vec<u8>) {
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

    let app = Application::new(&context, dims);

    let _red: Vec<u8> = vec![255, 0, 0];
    let _green: Vec<f32> = vec![0.0, 1.0, 0.0];
    let _blue: Vec<u8> = vec![0, 0, 255];

    // let poly = IrregularPolygon::new_from_path(vec![
    //     100.0, 100.0, 200.0, 100.0, 200.0, 200.0, 100.0, 200.0
    // ]);

    let pentagon = RegularPolygon::new(100.0 , 7);


    // app.add_shape(&pentagon);
    pentagon.translate(-70.0, 00.0);

    // app.draw_colored_shape(&poly.get_geom(), &blue);


    let tex = load_texture_image(Rc::new(context), get_img().as_str());

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        app.clear_all();
        app.draw_textured_shape(&pentagon.get_geom(), &tex);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> ()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


