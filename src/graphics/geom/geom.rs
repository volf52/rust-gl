use crate::math::Matrix;

pub struct Geom {
    pub vertices: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub u_mat: Matrix,
    pub mode: u32,
    pub vertex_count: i32,
}

impl Geom {
    pub fn new(vertices: Vec<f32>, u_mat: Matrix, mode: u32, vertex_count: i32) -> Self {
        let tex_coords = calc_tex_coords(&vertices);

        Geom {
            vertices,
            tex_coords,
            u_mat,
            mode,
            vertex_count,
        }
    }
    pub fn rotate(&mut self, angle: f32) {
        self.u_mat = self.u_mat.rotate(&angle);
    }
    pub fn translate(&mut self, tx: &f32, ty: &f32) {
        self.u_mat = self.u_mat.translate(&tx, &ty);
    }
}

fn calc_tex_coords(vertices: &Vec<f32>) -> Vec<f32> {
    let xs: Vec<f32> = vertices
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, e)| e.clone())
        .collect();

    let ys: Vec<f32> = vertices
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, e)| e.clone())
        .collect();

    let max = (
        xs.iter().cloned().fold(0. / 0., f32::max),
        ys.iter().cloned().fold(0. / 0., f32::max),
    );
    let min = (
        xs.iter().cloned().fold(0. / 0., f32::min),
        ys.iter().cloned().fold(0. / 0., f32::min),
    );

    let diff = (max.0 - min.0, max.1 - min.1);


    vertices
        .iter()
        .enumerate()
        .map(|f| {
            if f.0 % 2 != 0 {
                (f.1 - min.0) / diff.0
            } else {
                (f.1 - min.1) / diff.1
            }
        })
        .collect()
}
