use crate::graphics::{Geom, Shape};
use crate::math::bounding_rect::Bounded;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Circle {
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Circle {
    pub fn new_at(x: f32, y: f32, radius: f32, color_or_texture: &impl TextureGen) -> Self {
        let vertex_count = 200;

        let geom = Geom::build_geom(x, y, radius, radius, vertex_count, color_or_texture);

        Circle { radius, geom }
    }

    pub fn new_at_origin(radius: f32, color_or_texture: &impl TextureGen) -> Self {
        Circle::new_at(0.0, 0.0, radius, color_or_texture)
    }
}

impl Shape for Circle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl Bounded for Circle {
    fn contains(&self, x: f32, y: f32) -> bool {
        let (c_x, c_y) = self.get_center();

        match self.radius {
            r2 if r2 <= 0.0 => false,
            r2 => {
                let dx = (x - c_x).powi(2);
                let dy = (y - c_y).powi(2);

                (dx + dy) <= r2.powi(2)
            }
        }
    }
}
