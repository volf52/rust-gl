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
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: f32,

    geom: Rc<RefCell<Geom>>,
}

impl Ellipse {
    pub fn new(x: i32, y: i32, width: f32, height: f32, color: &Vec<f32>) -> Self {
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

    pub fn new_at_origin(width: f32, height: f32, color: &Vec<f32>) -> Self {
        Self::new(0, 0, width, height, color)
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

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32, color: &Vec<f32>) -> Self {
        let vertex_count = 200;
        let vertices = calc_n_vertices(radius, radius, vertex_count);
        let color_data = color_n_vertices(color, vertex_count);

        let geom = Rc::new(RefCell::new(Geom {
            vertices,
            color: color_data,
            u_mat: Matrix::new(),
            mode: WebGl2RenderingContext::TRIANGLE_FAN,
            vertex_count: vertex_count as i32,
        }));

        Circle { x, y, radius, geom }
    }

    pub fn new_at_origin(radius: f32, color: &Vec<f32>) -> Self {
        Self::new(0, 0, radius, color)
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
