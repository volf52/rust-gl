use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{WebGl2RenderingContext, WebGlTexture};

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

        let geom = Rc::new(RefCell::new(Geom::new(
            vertices,
            Matrix::new(),
            WebGl2RenderingContext::TRIANGLES,
            3,
        )));

        Triangle { size, geom }
    }
}

impl Shape for Triangle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
