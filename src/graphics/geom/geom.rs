use crate::math::Matrix;

pub struct Geom {
    pub vertices: Vec<f32>,
    pub color: Vec<f32>,
    pub u_mat: Matrix,
    pub mode: u32,
    pub vertex_count: i32,
}

impl Geom {
    pub fn rotate(&mut self, angle: f32) {
        self.u_mat = self.u_mat.rotate(&angle);
    }
    pub fn translate(&mut self, tx: &f32, ty: &f32) {
        self.u_mat = self.u_mat.translate(&tx, &ty);
    }
}
