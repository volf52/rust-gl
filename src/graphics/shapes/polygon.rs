use crate::graphics::{Geom, Shape};
use crate::math::BoundingRect;
use crate::textures::utils::{TextureGen, TextureOrColor};
use std::cell::RefCell;
use std::ops::AddAssign;
use std::rc::Rc;

pub struct RegularPolygon {
    pub x: i32,
    pub y: i32,

    pub radius: f32,
    pub sides: usize,
    geom: Rc<RefCell<Geom>>,
}

pub struct IrregularPolygon {
    pub x: i32,
    pub y: i32,

    pub width: f32,
    pub height: f32,
    pub sides: usize,
    geom: Rc<RefCell<Geom>>,
}

impl RegularPolygon {
    pub fn new(
        x: i32,
        y: i32,
        radius: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let sides = n_sides.max(3);
        let geom = Geom::build_geom(x as f32, y as f32, radius, radius, sides, color_or_texture);

        RegularPolygon {
            x,
            y,
            radius,
            sides,
            geom,
        }
    }

    pub fn new_at_origin(radius: f32, n_sides: usize, color_or_texture: &impl TextureGen) -> Self {
        Self::new(0, 0, radius, n_sides, color_or_texture)
    }
}

impl Shape for RegularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        todo!()
    }
}

impl IrregularPolygon {
    pub fn new(
        x: i32,
        y: i32,
        width: f32,
        height: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        let sides = n_sides.max(3);
        let geom = Geom::build_geom(x as f32, y as f32, width, height, sides, color_or_texture);

        IrregularPolygon {
            x,
            y,
            width,
            height,
            sides,
            geom,
        }
    }

    pub fn new_from_path(vertices: Vec<f32>, color_or_texture: &impl TextureGen) -> Self {
        let sides = vertices.len() / 2;

        let xs: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| *e)
            .collect();

        let ys: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, e)| *e)
            .collect();

        let width = xs.iter().cloned().fold(f32::NAN, f32::max)
            - xs.iter().cloned().fold(f32::NAN, f32::min);
        let height = ys.iter().cloned().fold(f32::NAN, f32::max)
            - ys.iter().cloned().fold(f32::NAN, f32::min);

        let geom = Geom::build_geom(0.0, 0.0, width, height, sides, color_or_texture);

        IrregularPolygon {
            x: 0,
            y: 0,
            width,
            height,
            sides,
            geom,
        }
    }

    pub fn new_at_origin(
        width: f32,
        height: f32,
        n_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Self {
        Self::new(0, 0, width, height, n_sides, color_or_texture)
    }

    pub fn translate2(&mut self, tx: f32, ty: f32) {
        self.geom.borrow_mut().translate(tx, ty);
        self.x.add_assign(tx as i32);
        self.y.add_assign(ty as i32);
    }
}

impl Shape for IrregularPolygon {
    fn get_geom(&self) -> Rc<RefCell<Geom>> {
        self.geom.clone()
    }

    fn get_bounds(&self) -> BoundingRect {
        todo!()
    }

    fn contains(&self, _x: f32, _y: f32) -> bool {
        todo!()
    }
}
