use crate::graphics::shapes::utils::{calc_n_vertices, color_n_vertices};
use crate::graphics::{Geom, Shape};
use crate::math::{BoundingRect, Matrix};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub width: f32,
    pub height: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new(width: f32, height: f32, color: &[f32]) -> Self {
        Self::new_at(0, 0, width, height, color)
    }

    pub fn new_at(x: i32, y: i32, width: f32, height: f32, color: &[f32]) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(width, height, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::translation(x as f32, y as f32),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

        Ellipse {
            x,
            y,
            width,
            height,
            geom,
        }
    }
}

impl Shape for Ellipse {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        let x_pos = (self.x as f32) - self.width;
        let y_pos = (self.y as f32) - self.height;

        BoundingRect::new(x_pos, y_pos, self.width, self.height)
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            _ => {
                let norm_x = ((x - self.x as f32) / self.width).powi(2);
                let norm_y = ((y - self.y as f32) / self.height).powi(2);

                (norm_x + norm_y) <= 1.0
            }
        }
    }
}
