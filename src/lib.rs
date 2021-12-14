use std::{cell::RefCell, rc::Rc};

use graphics::Container;
use math::bounding_rect::Bounded;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{
    Circle, IrregularPolygon, Rectangle, RegularPolygon, Shape, Triangle,
};

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

    let red: Vec<u8> = vec![255, 0, 0];
    let _green: Vec<u8> = vec![0, 255, 0];
    let blue: Vec<u8> = vec![0, 0, 255];

    let tex = app.tex_from_img("../assets/test.jpg");

    let c = Circle::new_at_origin(100.0, &red);

    let mut container = Container::default();

    // console_log!("Center for c: {:?}", c.get_center()); // should be 0.0, 0.0

    // let p = (-50.0, 50.0);
    // console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // true
    // console_log!(
    //     "Contains in bounds {:?} for c: {}",
    //     p,
    //     c.contains_in_bounds(p.0, p.1)
    // ); // true

    // let p = (-90.0, -90.0);
    // console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // false
    // console_log!(
    //     "Contains in bounds {:?} for c: {}",
    //     p,
    //     c.contains_in_bounds(p.0, p.1)
    // ); // true

    // let p = (100.1, 100.0);
    // console_log!("Contains {:?} for c: {}", p, c.contains(p.0, p.1)); // false
    // console_log!(
    //     "Contains in bounds {:?} for c: {}",
    //     p,
    //     c.contains_in_bounds(p.0, p.1)
    // ); // false

    let c_bounds = c.get_bounds();
    let c_bounding_rect = Rectangle::new_at(
        c_bounds.x as i32,
        c_bounds.y as i32,
        c_bounds.width,
        c_bounds.height,
        &blue,
    );

    app.add_container(&container);

    c.rotate(0.3);
    c.scale(1.1, 2.1);
    c.move_by(10.0, 10.0);
    let c_current_center = c.get_center();
    console_log!("-----------");
    console_log!("C orig center: {:?}", c_current_center);
    c.scale(2.1, 1.1);
    c.move_to(110.1, 121.2);
    container.scale(0.4, 0.5);

    container.add_shape(&c);
    container.add_shape(&c_bounding_rect);

    let c_new_center = c.get_center();
    console_log!("C new center: {:?}", c_new_center); // should be 110.1, 121.2

    render_loop(move || {
        app.render();
        // tr.rotate_deg(0.5);
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
