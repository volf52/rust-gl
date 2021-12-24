// use crate::puts;
use image::{DynamicImage, EncodableLayout, Rgba};
use rusttype::{point, Font, Scale};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};

type Wgl2 = WebGl2RenderingContext;

pub fn create_text_texture(
    gl: &WebGl2RenderingContext,
    text: &str,
    font: &str,
    text_size: u32,
    color: &str,
    tx: f32,
    ty: f32,
) -> WebGlTexture {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
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
    ctx.set_font(&font_string);
    ctx.set_text_baseline("middle");
    ctx.set_text_align("center");

    ctx.set_fill_style(&JsValue::from_str(color));
    let text_width = ctx.measure_text(text).unwrap().width();
    // let canvas_width = text_width * 2.5;
    // let canvas_height = text_size * 2;

    let _err = ctx.fill_text(
        text,
        text_width as f64 + tx as f64,
        text_size as f64 * 2. + ty as f64,
    );
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
    texture
}

pub fn test_tex(gl: &WebGl2RenderingContext) -> WebGlTexture {
    let font_data = include_bytes!("../../assets/AnkaCoder-b.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(16.0);

    // The text to render
    let text = "Hello World";

    // Use a dark red colour
    let colour = (0, 0, 0);

    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();
    // let data: Vec<u8> = glyphs.iter().map(|f| f.draw(|x, y, v| v * 255.0)).collect();
    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs.clone() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    let texture = gl.create_texture().unwrap();

    gl.bind_texture(Wgl2::TEXTURE_2D, Some(&texture));
    let _err = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        Wgl2::TEXTURE_2D,
        0,
        Wgl2::RGBA as i32,
        image.width() as i32,
        image.height() as i32,
        0,
        Wgl2::RGBA,
        Wgl2::UNSIGNED_BYTE,
        Some(image.as_bytes()),
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
    texture
}
