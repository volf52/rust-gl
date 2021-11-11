use super::Geom;

pub trait Shape {
    fn get_geom(&self) -> Geom;
    fn rotate(&mut self, angle: f32);
}
