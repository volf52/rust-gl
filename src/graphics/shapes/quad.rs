use std::cell::RefCell;
use std::rc::Rc;

use crate::math::{BoundingRect, Matrix};

use crate::graphics::{Geom, Shape};
use web_sys::WebGl2RenderingContext;

#[derive(Clone)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,

    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: f32, height: f32) -> Self {
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
            u_mat: Matrix::translation(x as f32, y as f32),
            vertices,
            color,
            vertex_count: 4,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        };

        let rect = Rectangle {
            x,
            y,
            width,
            height,
            geom: Rc::new(RefCell::new(geom)),
        };

        rect
    }

    pub fn new_at_origin(width: f32, height: f32) -> Self {
        Self::new(0, 0, width, height)
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        BoundingRect::new(self.x as f32, self.y as f32, self.width, self.height)
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y)
    }
}
