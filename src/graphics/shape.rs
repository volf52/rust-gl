use crate::Geom;

pub enum Shape {
    Rectangle { width: f32, height: f32 },
    Square { size: f32},
    Triangle { size: f32 },
}

impl Shape {
    pub fn new(&self) -> Geom {
        match self {
            Shape::Rectangle { width, height } => {
                Geom::new_rectangle(width.clone(), height.clone())
            }
            Shape::Triangle { size } => Geom::new_triangle(size.clone()),
            Shape::Square { size } => Geom::new_rectangle(size.clone(), size.clone())
        }
    }
}
