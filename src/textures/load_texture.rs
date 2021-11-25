
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{
    HtmlImageElement, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture,
};

/// Load a new texture :)
///
/// To do so, the texture image needs to be loaded from the server first. This is done
/// asynchronously in Javascript so we can upload the image to the GPU only after the image
/// is received on the client.
///
/// One trick is to create first the texture with one single blue pixel, then add a callback to
/// load the texture when the image is loaded. See here: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Using_textures_in_WebGL
pub fn load_texture(
    gl: &WebGl2RenderingContext,
    img_src: &str,
) -> Result<WebGlTexture, JsValue> {
    let texture = gl.create_texture().expect("Cannot create gl texture");
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    let level = 0;
    let internal_format = WebGl2RenderingContext::RGBA;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = WebGl2RenderingContext::RGBA;
    let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;

    // Now upload single pixel.
    let pixel: [u8; 4] = [0, 0, 255, 255];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D,
        level,
        internal_format as i32,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&pixel),
    )?;

    let img = HtmlImageElement::new().unwrap();
    img.set_cross_origin(Some(""));

    img.set_src(img_src);

    let imgrc = Rc::new(img);

    let texture = texture;

    {
        let img = imgrc.clone();
        let texture = texture.clone();
        let gl = Rc::new(gl.clone());
        let a = Closure::wrap(Box::new(move || {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

            if let Err(e) = gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                level,
                internal_format as i32,
                src_format,
                src_type,
                &img,
            ) {
                // TODO better error handling...
                console::log_1(&e);
                return;
            }

            // different from webgl1 where we need the pic to be power of 2
            gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }) as Box<dyn FnMut()>);
        imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

        // Normally we'd store the handle to later get dropped at an appropriate
        // time but for now we want it to be a global handler so we use the
        // forget method to drop it without invalidating the closure. Note that
        // this is leaking memory in Rust, so this should be done judiciously!
        a.forget();
    }

    imgrc.set_src(img_src);

    Ok(texture)
}
