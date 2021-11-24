use std::cell::RefCell;
use std::rc::Rc;
use crate::math::Matrix;

use crate::graphics::{Geom, Shape};
use web_sys::{WebGl2RenderingContext, WebGlTexture};

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

        // let color: [u8; 3] = [0,0,255];

        let geom = Rc::new(RefCell::new(Geom::new(
            vertices,
            Matrix::new(),
            WebGl2RenderingContext::TRIANGLE_STRIP,
            4,
        )));

        let rect = Rectangle {
            width,
            height,
            geom,
        };

        rect
    }
}

impl Shape for Rectangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
