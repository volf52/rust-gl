use crate::graphics::shapes::utils::color_n_times;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub fn create_solid_texture(gl: &WebGl2RenderingContext, color: &Vec<u8>) -> WebGlTexture {
    let level = 0;
    let internal_format = WebGl2RenderingContext::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = WebGl2RenderingContext::RGBA;
    let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;
    let color_twice = color_n_times(&color, 2);
    let pixel = color_twice.as_slice();

    let texture = gl.create_texture();
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(texture.as_ref().unwrap()),
    );

    let _err = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D,
        level,
        internal_format,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&pixel),
    );

    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );

    texture.unwrap()
}
