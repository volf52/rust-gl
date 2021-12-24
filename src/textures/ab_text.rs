use ab_glyph::{point, Font, FontRef, FontVec, Glyph, Point, PxScale, ScaleFont};
use image::{DynamicImage, EncodableLayout, Rgba};
use web_sys::{WebGl2RenderingContext, WebGlTexture};

// const TEXT: &str = "This is ab_glyph rendered into a png!";
type Wgl2 = WebGl2RenderingContext;
fn draw_image<F: Font>(font: F) {
    // The font size to use
    let scale = PxScale::from(45.0);
    let text = "Hello Everyone This";
    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 50.0, text, &mut glyphs);

    // Use a dark red colour
    let colour = (150, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }
}

pub fn test_tex2(gl: &WebGl2RenderingContext) -> WebGlTexture {
    let font_data = include_bytes!("../../assets/AnkaCoder-b.ttf");
    // This only succeeds if collection consists of one font
    let font = FontRef::try_from_slice(font_data).unwrap();

    // The font size to use
    let scale = PxScale::from(16.0);

    let scaled_font = font.as_scaled(scale);
    // The text to render
    let text = "Hello World";

    // Use a dark red colour
    let colour = (0, 0, 0);

    let mut glyphs: Vec<Glyph> = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), 50.0, text, &mut glyphs);

    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };
    // // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();
    // let data: Vec<u8> = glyphs.iter().map(|f| f.draw(|x, y, v| v * 255.0)).collect();
    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
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
pub fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
