use web_sys::WebGlTexture;

pub enum TextureOrColor {
    Texture(WebGlTexture),
    Color(Vec<u8>),
}

pub trait TextureGen {
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

