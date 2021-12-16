use std::{cell::RefCell, rc::Rc};

use graphics::Container;
use math::bounding_rect::Bounded;
use utils::{console_error, console_log};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::core::application::{Application, CanvasDimensions};
use crate::graphics::shapes::{
    Circle, Ellipse, IrregularPolygon, Rectangle, RegularPolygon, Shape, Triangle,
};
use crate::math::Matrix;

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
    let green: Vec<u8> = vec![180, 180, 180];
    let blue: Vec<u8> = vec![0, 0, 255];

    let tex = app.tex_from_img("../assets/test.jpg");

    // let c = Ellipse::new_at_origin(150.0, 75.0, &red);
    let pth = vec![0.0, 0.0, 50.0, 50.0, 150.0, 100.0, -100.0, 100.0];
    let c = IrregularPolygon::new_from_path(&pth, &red);

    let mut container = Container::default();

    app.add_container(&container);

    c.rotate_deg(5.0);
    c.move_by(10.0, 10.0);
    c.scale(1.1, 1.1);

    let c_bounding_rect = c.get_bounds();
    c_bounding_rect.set_texture(&blue);

    let p = (180.0, 150.0);
    console_log!("Point: {:?}", p);
    let p_inv = c.get_geom().borrow().u_mat.inverse_affine_point(p.0, p.1);
    console_log!("Inv Point: {:?}", p_inv);
    console_log!(
        "Contains in bounds (false): {:?}",
        c.contains_in_bounds(p.0, p.1)
    ); // should be false

    let p = (120.0, 150.0);
    console_log!("Point: {:?}", p);
    let p_inv = c.get_geom().borrow().u_mat.inverse_affine_point(p.0, p.1);
    console_log!("Inv Point: {:?}", p_inv);
    console_log!("Contains(false): {:?}", c.contains(p.0, p.1)); // should be false
    console_log!(
        "Contains in bounds (true): {:?}",
        c.contains_in_bounds(p.0, p.1)
    ); // should be true

    let p = (90.0, 30.0);
    console_log!("Point: {:?}", p);
    let p_inv = c.get_geom().borrow().u_mat.inverse_affine_point(p.0, p.1);
    console_log!("Inv Point: {:?}", p_inv);
    console_log!("Contains(true): {:?}", c.contains(p.0, p.1)); // should be true

    let c_normal = IrregularPolygon::new_from_path(&pth, &blue);

    let c_bound_normal =
        Rectangle::new_at_origin(c_bounding_rect.width, c_bounding_rect.height, &green);

    container.add_shape(&c_bounding_rect);
    container.add_shape(&c);

    container.add_shape(&c_bound_normal);
    container.add_shape(&c_normal);

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
