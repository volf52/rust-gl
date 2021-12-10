use image::EncodableLayout;
use typer::rusttype::Font;
use typer::{TextRenderer, Typer};
use web_sys::{WebGl2RenderingContext, WebGlTexture};
use xml::writer::XmlEvent;
type Wgl2 = WebGl2RenderingContext;

pub fn text_typer(gl: &WebGl2RenderingContext, xml_string: &str) -> WebGlTexture {
    let mut typer = Typer::new();
    let font_data = include_bytes!("../../assets/AnkaCoder-b.ttf");

    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let fonts = vec![("__".to_string(), font)];

    let blocks = typer.parse(&xml_string);
    let mut layout = TextRenderer::format(blocks, 1.0, fonts.as_slice());

    layout.width = 100.0;
    layout.height = 100.0;
    // // layout.x = -100.0;
    // layout.y = -150.0;

    let mut buffer = layout.create_buffer(&[255, 0, 0, 0]).unwrap();
    TextRenderer::render(&layout, &mut buffer);

    let img_buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::RgbaImage::from_vec(buffer.width as u32, buffer.height as u32, buffer.buffer)
            .unwrap();
    texture_from_image(gl, img_buf)
}

pub fn texture_from_image(
    gl: &WebGl2RenderingContext,
    image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> WebGlTexture {
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
        Some(&image.as_bytes()),
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
