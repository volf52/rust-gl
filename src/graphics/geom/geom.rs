use crate::math::Matrix;

use super::Geom;

impl Geom {
    pub fn position(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn color(&self) -> Vec<f32> {
        self.color.clone()
    }

    pub fn calculate_projection_mat(&self, mat: &Matrix) -> Matrix {
        let mat = Matrix::rotate(mat, self.rotation);
        // other stuff

        mat
    }
}
