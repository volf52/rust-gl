use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::bounding_rect::Bounded;
use crate::math::{BoundingRect, Matrix};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Circle {
    pub fn new(radius: f32, color: &[f32]) -> Self {
        Self::new_at(0, 0, radius, color)
    }

    pub fn new_at(x: i32, y: i32, radius: f32, color: &[f32]) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(radius, radius, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::translation(x as f32, y as f32),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

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
