use super::shape::Shape;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Square {
    pub vert_count: i32,
    position: [f32; 8],
    color: [f32; 12],
}

impl Shape for Square {}

#[wasm_bindgen]
impl Square {
    pub fn new(size: f32) -> Self {
        let left = -size;
        let right = size;
        let top = size;
        let bottom = -size;

        let position = [left, top, right, top, left, bottom, right, bottom];
        let color = [
            1.0, 0.4, 0.4, // vertex 1
            1.0, 0.0, 0.0, // vertex 2
            0.0, 1.0, 0.0, // vertex 3
            0.0, 0.0, 1.0, // vertex 4
        ];
        let vert_count = 4;

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
