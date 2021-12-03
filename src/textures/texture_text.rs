use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, WebGl2RenderingContext, WebGlTexture};

use crate::puts;

pub fn create_text_texture(
    gl: &WebGl2RenderingContext,
    text: &str,
    font: &str,
    text_size: u32,
    color: &str,
) -> WebGlTexture {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let texture = gl.create_texture().unwrap();

    let font_string = format!("{}px {}", text_size, font);
    ctx.set_fill_style(&JsValue::from_str(&color));

    ctx.set_font(&font_string);
    ctx.set_text_baseline("middle");
    ctx.set_text_align("center");
    let text_width = ctx.measure_text(text).unwrap().width();

    let canvas_width = text_width as u32 * 2;
    let canvas_height = text_size * 2;

    canvas.set_height(canvas_height);
    canvas.set_width(canvas_width);

    ctx.set_shadow_color("rgba(10, 160, 190, 1.0)");
    //    ctx.clear_rect(0.0, 0.0, 150 as f64, 26 as f64);
    let _err = ctx.fill_text(text, canvas_width as f64 / 2.5, canvas_height as f64 / 1.75);

    type Wgl2 = WebGl2RenderingContext;

    //gl.pixel_storei(Wgl2::UNPACK_FLIP_Y_WEBGL, 1);
    gl.bind_texture(Wgl2::TEXTURE_2D, Some(&texture));
    let _err = gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
        Wgl2::TEXTURE_2D,
        0,
        Wgl2::RGBA as i32,
        Wgl2::RGBA,
        Wgl2::UNSIGNED_BYTE,
        &canvas,
    );
    gl.tex_parameteri(
        Wgl2::TEXTURE_2D,
        Wgl2::TEXTURE_MIN_FILTER,
        Wgl2::LINEAR as i32,
    );
    gl.tex_parameteri(
        Wgl2::TEXTURE_2D,
        Wgl2::TEXTURE_WRAP_S,
        Wgl2::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        Wgl2::TEXTURE_2D,
        Wgl2::TEXTURE_WRAP_T,
        Wgl2::CLAMP_TO_EDGE as i32,
    );
    // g
    // gl.generate_mipmap(Wgl2::TEXTURE_2D);
    // gl.bind_texture(Wgl2::TEXTURE_2D, 0);
    texture
}
