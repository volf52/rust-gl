use dyn_clonable::*;

#[clonable]
pub trait Shape: Clone {
    fn vertex_count(&self) -> i32;
    fn position(&self) -> Vec<f32>;
    fn color(&self) -> Vec<f32>;
}