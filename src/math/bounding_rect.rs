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
            t if (x >= self.x && x < self.x + t.0) && (y >= self.y && y < self.y + t.1) => true,
            _ => false,
        }
    }
}

pub trait Bounded {
    fn get_bounds(&self) -> BoundingRect;
    fn contains(&self, x: f32, y: f32) -> bool;

    fn contains_in_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().contains(x, y)
    }
}
