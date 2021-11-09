use super::Geom;

impl Geom {
    pub fn position(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn color(&self) -> Vec<f32> {
        self.color.clone()
    }
}
