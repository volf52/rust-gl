use self::ShaderConstant::*;
use std::fmt::{self, Display};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderConstant {
    APosition,
    AColor,
    UProjection,
    UModel,
    VColor,
    USampler,
}

pub const ATTRIBUTES: [ShaderConstant; 2] = [APosition, AColor];
pub const UNIFORMS: [ShaderConstant; 2] = [UModel, UProjection];

impl Display for ShaderConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
