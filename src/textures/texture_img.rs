use image::{DynamicImage, GenericImageView};
use web_sys::{WebGl2RenderingContext, WebGlTexture};
use crate::puts;

pub fn texture_from_image(gl: &WebGl2RenderingContext, path: &str) -> WebGlTexture{
   let arr = image::open(path);
   texture_from_image_obj(gl, &arr.unwrap())
}

pub fn test_img(path: &str) {
    let arr = image::open(path);
    puts(&arr.err().unwrap().to_string().as_str())
 }

fn texture_from_image_obj(gl: &WebGl2RenderingContext, image: &DynamicImage) -> WebGlTexture {
    let level = 0;
    let internal_format = WebGl2RenderingContext::RGBA as i32;
    let width = image.width() as i32;
    let height = image.height() as i32;
    let border = 0;
    let src_format = WebGl2RenderingContext::RGBA;
    let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;
    let arr = image.to_rgb8().as_raw().clone();



    let texture = gl.create_texture();
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture.as_ref().unwrap()));

    let _err = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D,
        level,
        internal_format,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&arr),
    );
    
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);

    texture.unwrap()
}
