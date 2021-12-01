use crate::graphics::{Geom, Shape};
use crate::math::BoundingRect;
use crate::textures::utils::TextureGen;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32, color_or_texture: &impl TextureGen) -> Self {
        let vertex_count = 200;

        let geom = Geom::build_geom(
            x as f32,
            y as f32,
            radius,
            radius,
            vertex_count,
            color_or_texture,
        );

        Circle { x, y, radius, geom }
    }
}

impl Shape for Circle {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        let x_pos = (self.x as f32) - self.radius;
        let y_pos = (self.y as f32) - self.radius;
        let width_height = self.radius.powi(2);

        BoundingRect::new(x_pos, y_pos, width_height, width_height)
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        match self.radius.powi(2) {
            r2 if r2 <= 0.0 => false,
            r2 => {
                let dx = (self.x as f32 - x).powi(2);
                let dy = (self.y as f32 - y).powi(2);

                (dx + dy) <= r2
            }
        }
    }
}
