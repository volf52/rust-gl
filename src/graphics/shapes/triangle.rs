use std::cell::RefCell;
use std::rc::Rc;

use web_sys::WebGl2RenderingContext;

use crate::graphics::{Geom, Shape};
use crate::math::Matrix;

pub struct Triangle {
    pub size: f32,
    geom: Rc<RefCell<Geom>>,
}

impl Triangle {
    pub fn new(size: f32) -> Self {
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
            u_mat: Matrix::new(),
            vertices,
            color,
            vertex_count: 3,
            mode: WebGl2RenderingContext::TRIANGLES,
        }));

        Triangle { size, geom }
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn rotate(&self, angle: f32) {
        self.geom.borrow_mut().rotate(angle);
    }
}
