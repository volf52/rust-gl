use super::super::shape::Shape;
use super::Geom;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
pub struct Triangle {
    pub size: f32,
    pub rotation: f32,
}

#[wasm_bindgen]
impl Triangle {
    pub fn new(size: f32) -> Self {
        Triangle {
            size,
            rotation: 0.0,
        }
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Geom {
        let right = self.size / 2.0;
        let left = -right;
        let top = right;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom].to_vec();
        let color = [
            0.0, 0.0, 0.0, // vertex 1
            0.0, 0.0, 0.0, // vertex 2
            1.0, 0.0, 0.0, // vertex 3
        ]
        .to_vec();

        Geom {
            rotation: self.rotation,
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }
}
