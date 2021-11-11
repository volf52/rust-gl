use super::super::shape::Shape;
use super::Geom;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    width: f32,
    height: f32,
    pub rotation: f32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Rectangle {
            width,
            height,
            rotation: 0.0,
        }
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Geom {
        let right = self.width / 2.0;
        let left = -right;
        let top = self.height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = [
            1.0, 0.4, 0.4, // vertex 1
            1.0, 0.0, 0.0, // vertex 2
            0.0, 1.0, 0.0, // vertex 3
            0.0, 0.0, 1.0, // vertex 4
        ]
        .to_vec();

        Geom {
            rotation: self.rotation,
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }
}
