use crate::textures::utils::TextureGen;

use super::IrregularPolygon;

pub enum Shape {
    Triangle(f32),
    Square(f32),
    Circle(f32),
    Polygon(f32, usize),
}

pub trait ConstructAtOrigin {
    fn new_at_origin(&self) -> IrregularPolygon;
}

pub trait Construct {
    fn new(&self, x: i32, y: i32) -> IrregularPolygon;
}

pub trait ConstructTextured {
    fn new_at_origin_textured(&self, mask: impl TextureGen) -> IrregularPolygon;
    fn new_textured(&self, x: i32, y: i32, mask: &impl TextureGen) -> IrregularPolygon;
}

impl ConstructAtOrigin for Shape {
    fn new_at_origin(&self) -> IrregularPolygon {
        let mask: Vec<u8> = vec![0, 0, 0];
        match self {
            &Shape::Circle(x) => IrregularPolygon::new_at_origin(x, x, 200, &mask),
            &Shape::Square(x) => IrregularPolygon::new_at_origin(x, x, 4, &mask),
            &Shape::Triangle(x) => IrregularPolygon::new_at_origin(x, x, 3, &mask),
            &Shape::Polygon(x, n) => IrregularPolygon::new_at_origin(x, x, n, &mask),
        }
    }
}

impl Construct for Shape {
    fn new(&self, tx: i32, ty: i32) -> IrregularPolygon {
        let mask: Vec<u8> = vec![0, 0, 0];
        match self {
            &Shape::Circle(x) => IrregularPolygon::new(tx, ty, x, x, 200, &mask),
            &Shape::Square(x) => IrregularPolygon::new(tx, ty, x, x, 4, &mask),
            &Shape::Triangle(x) => IrregularPolygon::new(tx, ty, x, x, 3, &mask),
            &Shape::Polygon(x, n) => IrregularPolygon::new(tx, ty, x, x, n, &mask),
        }
    }
}

impl ConstructTextured for Shape {
    fn new_at_origin_textured(&self, mask: impl TextureGen) -> IrregularPolygon {
        match self {
            &Shape::Circle(x) => IrregularPolygon::new_at_origin(x, x, 200, &mask),
            &Shape::Square(x) => IrregularPolygon::new_at_origin(x, x, 4, &mask),
            &Shape::Triangle(x) => IrregularPolygon::new_at_origin(x, x, 3, &mask),
            &Shape::Polygon(x, n) => IrregularPolygon::new_at_origin(x, x, n, &mask),
        }
    }
    fn new_textured(&self, tx: i32, ty: i32, mask: &impl TextureGen) -> IrregularPolygon {
        match self {
            &Shape::Circle(x) => IrregularPolygon::new(tx, ty, x, x, 200, mask),
            &Shape::Square(x) => IrregularPolygon::new(tx, ty, x, x, 4, mask),
            &Shape::Triangle(x) => IrregularPolygon::new(tx, ty, x, x, 3, mask),
            &Shape::Polygon(x, n) => IrregularPolygon::new(tx, ty, x, x, n, mask),
        }
    }
}
