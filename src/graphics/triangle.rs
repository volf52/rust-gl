use crate::graphics::shape::Shape;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Triangle {
    vert_count: i32,
    position: [f32; 6],
    color: [f32; 9],
}

#[wasm_bindgen]
impl Triangle {
    pub fn new(size: f32) -> Self {
        let left = -size;
        let right = size;
        let top = size;
        let bottom = -size;

        let position = [left, top, right, top, left, bottom];
        let color = [
            0.0, 0.0, 0.0, // vertex 1
            0.0, 0.0, 0.0, // vertex 2
            1.0, 0.0, 0.0, // vertex 3
        ];
        let vert_count = 3;

        Triangle {
            position,
            color,
            vert_count,
        }
    }
}

impl Shape for Triangle {
    fn vertex_count(&self) -> i32 {
        return self.vert_count;
    }

    fn position(&self) -> Vec<f32> {
        self.position.to_vec()
    }

    fn color(&self) -> Vec<f32> {
        self.color.to_vec()
    }

    fn mode(&self) -> u32 {
        WebGl2RenderingContext::TRIANGLES
    }
}
