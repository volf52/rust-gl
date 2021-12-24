use web_sys::WebGlTexture;

#[derive(Debug, Clone)]
pub enum TextureOrColor {
    Texture(WebGlTexture),
    Color(Vec<u8>),
}

pub trait TextureGen: Sized {
    fn to_enum(&self) -> TextureOrColor;
}

impl TextureGen for WebGlTexture {
    fn to_enum(&self) -> TextureOrColor {
        TextureOrColor::Texture(self.clone())
    }
}

impl TextureGen for Vec<u8> {
    fn to_enum(&self) -> TextureOrColor {
        TextureOrColor::Color(self.clone())
    }
}
