use std::cell::RefCell;
use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use crate::graphics::{Geom, Shape};
use crate::math::{BoundingRect, Matrix};

pub struct Triangle {
    pub x: i32,
    pub y: i32,

    pub size: f32,
    geom: Rc<RefCell<Geom>>,
}

impl Triangle {
    pub fn new(size: f32, color: &[f32]) -> Self {
        Self::new_at(0, 0, size, color)
    }

    pub fn new_at(x: i32, y: i32, size: f32, color: &[f32]) -> Self {
        let right = size / 2.0;
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

        let geom = Rc::new(RefCell::new(Geom {
            u_mat: Matrix::translation(x as f32, y as f32),
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }));

        Triangle { size, geom, x, y }
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        todo!()
    }
}
