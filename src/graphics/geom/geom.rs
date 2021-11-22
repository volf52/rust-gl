use crate::math::Matrix;

// #[wasm_bindgen]
pub struct Geom {
    pub vertices: Vec<f32>,
    pub color: Vec<f32>,
    pub mode: u32,
    pub vertex_count: i32,
    pub matrix: Matrix,
}

impl Geom {
    pub fn position(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn color(&self) -> Vec<f32> {
        self.color.clone()
    }

    pub fn rotate(&self, angle: f32) -> Self {
        Geom {
            vertices: self.vertices.clone(),
            color: self.color.clone(),
            mode: self.mode.clone(),
            vertex_count: self.vertex_count.clone(),
            matrix: self.matrix.rotate(&angle),
        }
    }

    pub fn rotate_deg(&self, angle: f32) -> Self {
        Geom {
            vertices: self.vertices.clone(),
            color: self.color.clone(),
            mode: self.mode.clone(),
            vertex_count: self.vertex_count.clone(),
            matrix: self.matrix.rotate(&angle.to_radians()),
        }
    }

    pub fn translate(&self, tx: f32, ty:f32) -> Self {
        Geom {
            vertices: self.vertices.clone(),
            color: self.color.clone(),
            mode: self.mode.clone(),
            vertex_count: self.vertex_count.clone(),
            matrix: self.matrix.translate(&tx, &ty),
        }
    }
}
