use web_sys::WebGl2RenderingContext;
use std::rc::Rc;
use crate::RefCell;
use crate::graphics::Geom;
use crate::math::Matrix;
use std::f32::consts::PI;

pub fn calc_n_vertices(width: f32, height: f32, num_sides: usize) -> Vec<f32> {
    let k = (PI * 2.0) / num_sides as f32;
    (0..num_sides).fold(vec![], |acc, x| {
        acc.iter()
            .copied()
            .chain([width * (k * x as f32).cos(), height * (k * x as f32).sin()])
            .collect()
    })
}

pub fn color_n_times(unit_color: &Vec<u8>, num_vertices: usize) -> Vec<u8> {
    unit_color
        .iter()
        .cycle()
        .take(unit_color.len() * num_vertices)
        .map(|f| f.clone())
        .collect()
}

pub fn build(width: f32, height: f32, no_sides: usize) -> Rc<RefCell<Geom>> {
        let vertices = calc_n_vertices(width, height, no_sides);

        Rc::new(RefCell::new(Geom::new(
            vertices,
             Matrix::new(),
             WebGl2RenderingContext::TRIANGLE_FAN,
             no_sides as i32,
    )))
}


