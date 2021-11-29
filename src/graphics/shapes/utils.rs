use std::f32::consts::PI;

pub fn calc_n_vertices(width: f32, height: f32, num_sides: usize) -> Vec<f32> {
    let k = (PI * 2.0) / num_sides as f32;
    (0..num_sides).fold(vec![], |acc, x| {
        acc.iter()
            .cloned()
            .chain([width * (k * x as f32).cos(), height * (k * x as f32).sin()])
            .collect()
    })
}

pub fn color_n_vertices(unit_color: &[f32], num_vertices: usize) -> Vec<f32> {
    unit_color
        .iter()
        .cycle()
        .take(unit_color.len() * num_vertices)
        .cloned()
        .collect()
}
