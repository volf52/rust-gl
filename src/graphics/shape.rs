use std::f32::consts::PI;
use super::geom::geom::Geom;
pub trait Drawing {
    fn draw_shape(&self) -> Geom;
}
pub trait Transform {
    fn get_geom(&self) -> Geom;
}

pub fn calc_n_vertices(width: &f32, height: &f32, no_sides: u32) -> Vec<f32> {
    let k = (PI * 2.0) / no_sides as f32;
    (0..no_sides).fold(vec![], |acc, x| {
        acc.iter()
            .copied()
            .chain([width * (k * x as f32).cos(), height * (k * x as f32).sin()])
            .collect()
    })
}

pub fn color_n_vertices(unit_color: &Vec<f32>, no_vertices: usize) -> Vec<f32> {
    unit_color
        .iter()
        .cycle()
        .take(unit_color.len() * no_vertices)
        .map(|f| f.clone())
        .collect()
}

// pub fn rotate<T: Drawing>(shape: &T) -> T {
//     shape
// }