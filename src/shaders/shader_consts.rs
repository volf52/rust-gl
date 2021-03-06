use self::ShaderConstant::*;
use std::fmt::{self, Display};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderConstant {
    APosition,
    ATextureCoord,
    UProjection,
    UModel,
    VTextureCoord,
    VTextureCoord2,
    USampler,
}

pub const ATTRIBUTES: [ShaderConstant; 2] = [APosition, ATextureCoord];
pub const UNIFORMS: [ShaderConstant; 3] = [UModel, UProjection, USampler];

impl Display for ShaderConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
