
use crate::graphics::{Geom, Shape};
use crate::math::Matrix;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext};
use crate::graphics::shapes::utils::build;

pub struct RegularPolygon {
    pub radius: f32,
    pub sides: usize,
    geom: Rc<RefCell<Geom>>,
}

pub struct IrregularPolygon {
    pub width: f32,
    pub height: f32,
    pub sides: usize,
    geom: Rc<RefCell<Geom>>,
}

impl RegularPolygon {
    pub fn new(radius: f32, n_sides: usize) -> Self {
        let sides = n_sides.max(3);
        let geom = build(radius, radius, sides);

        RegularPolygon {
            radius,
            sides,
            geom,
        }
    }
}

impl Shape for RegularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}

impl IrregularPolygon {
    pub fn new(width: f32, height: f32, n_sides: usize) -> Self {
        let sides = n_sides.max(3);
        let geom = build(width, height, sides);

        IrregularPolygon {
            width,
            height,
            sides,
            geom,
        }
    }

    pub fn new_from_path(vertices: Vec<f32>) -> Self {
        // let color_data = color_n_vertices(color, vertices.len());
        let sides = vertices.len() / 2;

        let xs: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, e)| e.clone())
            .collect();

        let ys: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| e.clone())
            .collect();

        let width =
            xs.iter().cloned().fold(0. / 0., f32::max) - xs.iter().cloned().fold(0. / 0., f32::min);
        let height =
            ys.iter().cloned().fold(0. / 0., f32::max) - ys.iter().cloned().fold(0. / 0., f32::min);

        let geom = Rc::new(RefCell::new(Geom::new(
            vertices,
            Matrix::new(),
            WebGl2RenderingContext::TRIANGLE_FAN,
            sides as i32,
        )));

        IrregularPolygon {
            width,
            height,
            sides,
            geom,
        }
    }
}

impl Shape for IrregularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }
}
