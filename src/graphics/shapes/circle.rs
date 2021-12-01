use crate::graphics::{Geom, Shape};
use crate::math::bounding_rect::Bounded;
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
    pub fn new_at(x: i32, y: i32, radius: f32, color_or_texture: &impl TextureGen) -> Self {
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
}

impl Bounded for Circle {
    fn get_bounds(&self) -> BoundingRect {
        let (x_pos, y_pos) = self.get_center();
        let width_height = self.radius * 2.0;

        BoundingRect::new(x_pos, y_pos, width_height, width_height)
    }

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
