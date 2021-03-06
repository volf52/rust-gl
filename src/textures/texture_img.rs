use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};

use std::borrow::BorrowMut;

pub fn load_texture_image(gl: &WebGl2RenderingContext, src: &str) -> WebGlTexture {
    let gl = gl.clone();
    let texture = gl.create_texture().unwrap();
    let tex = texture.clone();
    let image = HtmlImageElement::new().unwrap();
    let mut image_clone = image.clone();

    let onload = Closure::wrap(Box::new(move || {
        let level = 0;
        let internal_format = WebGl2RenderingContext::RGBA as i32;
        let src_format = WebGl2RenderingContext::RGBA;
        let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let _err = gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            level,
            internal_format,
            src_format,
            src_type,
            &image,
        );
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
    }) as Box<dyn Fn()>);

    let image = image_clone.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(src);

    onload.forget();
    tex
}
