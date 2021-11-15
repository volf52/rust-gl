use crate::Geom;
pub enum Shape {
    Rectangle { width: f32, height: f32, color: Vec<f32> },
    Square { size: f32, color: Vec<f32> },
    Triangle { size: f32, color: Vec<f32> },
    Circle { radius: f32, color: Vec<f32> },
    Ellipse { width: f32, height: f32, color: Vec<f32> },
    RegularPolygon { radius: f32, sides: usize, color: Vec<f32> }
}

impl Shape {
    pub fn new(&self) -> Geom {
        match self {
            Shape::Rectangle { width, height , color} => Geom::new_rectangle(width.clone(), height.clone(), color.clone()),
            Shape::Triangle { size , color} => Geom::new_triangle(size.clone(), color.clone()),
            Shape::Square { size , color} => Geom::new_rectangle(size.clone(), size.clone(), color.clone()),
            Shape::Circle { radius , color} => Geom::new_ellipse(radius.clone(), radius.clone(), color.clone()),
            Shape::Ellipse { width, height, color} => Geom::new_ellipse(width.clone(), height.clone(), color.clone()),
            Shape::RegularPolygon { radius, sides, color } => 
                                            Geom::new_polygon(radius.clone(), sides.clone(), color.clone())
        }
    }
}
