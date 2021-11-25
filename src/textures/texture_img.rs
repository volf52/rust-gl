// use image::{DynamicImage, GenericImageView};
use std::rc::Rc;

use web_sys::{HtmlImageElement, WebGl2RenderingContext, WebGlTexture};
use wasm_bindgen::{JsCast, prelude::*};

use std::borrow::BorrowMut;


pub fn get_img() -> String {
    img_to_src().unwrap()
}


pub fn load_texture_image(gl: Rc<WebGl2RenderingContext>, src: &str) -> WebGlTexture {

    let texture = gl.create_texture().unwrap();
    let tex = texture.clone();
    let image = HtmlImageElement::new().unwrap();
    let mut image_clone = image.clone();
    // image.set_src(src);

    let onload = Closure::wrap(Box::new(move || {

        let level = 0;
        let internal_format = WebGl2RenderingContext::RGBA as i32;
        // let border = 0;
        let src_format = WebGl2RenderingContext::RGBA;
        let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;
    
        // gl.active_texture(WebGl2RenderingContext::TEXTURE0);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
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

#[wasm_bindgen(module = "/image.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn img_to_src() -> Result<String, JsValue>;
}