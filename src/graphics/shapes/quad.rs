use std::cell::RefCell;
use std::rc::Rc;

use crate::math::Matrix;

use crate::graphics::{Geom, Shape};
use web_sys::WebGl2RenderingContext;

#[derive(Clone)]
pub struct Rectangle {
    width: f32,
    height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        let right = width / 2.0;
        let left = -right;
        let top = height / 2.0;
        let bottom = -top;

        let vertices = [left, top, right, top, left, bottom, right, bottom].to_vec();
        let color = [
            1.0, 0.4, 0.4, // vertex 1
            1.0, 0.0, 0.0, // vertex 2
            0.0, 1.0, 0.0, // vertex 3
            0.0, 0.0, 1.0, // vertex 4
        ]
        .to_vec();

        let geom = Geom {
            u_mat: Matrix::new(),
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        };

        let rect = Rectangle {
            width,
            height,
            geom: Rc::new(RefCell::new(geom)),
        };

        rect
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn rotate(&self, angle: f32) {
        self.geom.borrow_mut().rotate(angle);
    }
}
