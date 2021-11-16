use crate::Geom;
pub trait Drawing {
    fn draw_shape(&self) -> Geom;
}