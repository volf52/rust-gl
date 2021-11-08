
use wasm_bindgen::prelude::*;
use crate::graphics::shape::Shape;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Square {
    vert_count: i32,
    position: [f32; 8],
    color: [f32; 12],
}

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
}

impl Shape for Square {
    fn vertex_count(&self) -> i32 {
        return self.vert_count;
    }

    fn position(&self) -> Vec<f32> {
        self.position.to_vec()
    }

    fn color(&self) -> Vec<f32> {
        self.color.to_vec()
    }

}
