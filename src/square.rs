use crate::shape::Shape;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Square {
    pub vert_count: i32,
    position: [f32; 12],
    color: [f32; 3],
}

impl Shape for Square {}

#[wasm_bindgen]
impl Square {
    pub fn new(size: f32) -> Self {
        let left = -size;
        let right = size;
        let top = size;
        let bottom = -size;

        let position = [
            left, bottom, right, bottom, left, top, // Triangle 1
            left, top, right, bottom, right, top, // Triangle 2
        ];
        let color = [1.0, 0.4, 0.4];
        let vert_count = 6;

        Square {
            position,
            color,
            vert_count,
        }
    }

    pub fn position(&self) -> Vec<f32> {
        self.position.to_vec()
    }

    pub fn color(&self) -> Vec<f32> {
        self.color.to_vec()
    }
}
