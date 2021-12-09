use crate::graphics::Shape;

#[derive(Debug, Clone)]
pub struct BoundingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PartialEq for BoundingRect {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl BoundingRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        BoundingRect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        ((self.x - other.x).abs() < f32::EPSILON)
            && ((self.y - other.y).abs() < f32::EPSILON)
            && ((self.width - other.width).abs() < f32::EPSILON)
            && ((self.height - other.height).abs() < f32::EPSILON)
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        match (self.width, self.height) {
            t if t.0 <= 0.0 || t.1 <= 0.0 => false,
            t if ((x - self.x).abs() <= (t.0 / 2.0)) && ((y - self.y).abs() <= (t.1 / 2.0)) => true,
            _ => false,
        }
    }
}

pub trait Bounded: Shape {
    fn contains(&self, x: f32, y: f32) -> bool;

    fn get_bounds(&self) -> BoundingRect {
        let (x_pos, y_pos) = self.get_center();
        let (scale_x, scale_y) = self.get_scale();

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        self.get_geom()
            .borrow()
            .vertices
            .chunks(2)
            .for_each(|chunk| {
                let x = chunk[0];
                let y = chunk[1];

                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            });

        let width = (max_x - min_x) * scale_x;
        let height = (max_y - min_y) * scale_y;

        BoundingRect::new(x_pos, y_pos, width, height)
    }

    fn contains_in_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y)
    }
}
