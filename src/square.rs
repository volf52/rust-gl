use crate::shape::Shape;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Square {
    position: Vec<f32>,
    texcoord: Vec<f32>,
    normal: Vec<f32>,
    color: Vec<u8>,
    indices: Vec<u8>,
}

impl Shape for Square {}

#[wasm_bindgen]
impl Square {
    pub fn new(_size: i32) -> Self {
        // TODO: Update data using size
        Square {
            position: Vec::new(),
            texcoord: Vec::new(),
            normal: Vec::new(),
            color: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn position(&self) -> Vec<f32> {
        self.position.clone()
    }

    pub fn texcoord(&self) -> Vec<f32> {
        self.texcoord.clone()
    }

    pub fn normal(&self) -> Vec<f32> {
        self.normal.clone()
    }

    pub fn color(&self) -> Vec<u8> {
        self.color.clone()
    }

    pub fn indices(&self) -> Vec<u8> {
        self.indices.clone()
    }
}
