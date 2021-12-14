use web_sys::WebGl2RenderingContext;

use crate::{
    graphics::shapes::utils::calc_n_vertices,
    math::Matrix,
    textures::utils::{TextureGen, TextureOrColor},
};
use std::{cell::RefCell, rc::Rc};

pub struct Geom {
    pub vertices: Vec<f32>,   // vertex data
    pub tex_coords: Vec<f32>, // texture vertices
    pub texture_data: TextureOrColor,
    pub u_mat: Matrix,
    pub mode: u32,
    pub vertex_count: i32,
}

impl Default for Geom {
    fn default() -> Self {
        Geom {
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            u_mat: Matrix::new(),
            mode: 0,
            vertex_count: 0,
            texture_data: TextureOrColor::Color(vec![]),
        }
    }
}

impl Geom {
    pub fn new(
        vertices: &[f32],
        u_mat: Matrix,
        mode: u32,
        vertex_count: i32,
        mask: &impl TextureGen,
    ) -> Self {
        let tex_coords = Geom::calc_tex_coords(vertices);
        let texture_data = mask.to_enum();

        Geom {
            vertices: vertices.to_vec(),
            tex_coords,
            u_mat,
            mode,
            vertex_count,
            texture_data,
        }
    }

    pub fn build_geom(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        no_sides: usize,
        color_or_texture: &impl TextureGen,
    ) -> Rc<RefCell<Geom>> {
        let vertices = calc_n_vertices(width, height, no_sides);

        Rc::new(RefCell::new(Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLE_FAN,
            no_sides as i32,
            color_or_texture,
        )))
    }

    pub fn set_texture(&mut self, text_gen: &impl TextureGen) {
        self.texture_data = text_gen.to_enum();
    }

    pub fn rotate(&mut self, angle: f32) {
        self.u_mat = self.u_mat.rotate(angle);
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.u_mat = self.u_mat.translate(tx, ty);
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.u_mat = self.u_mat.scale(x, y);
    }

    pub fn calc_tex_coords(vertices: &[f32]) -> Vec<f32> {
        let xs: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, e)| *e)
            .collect();

        let ys: Vec<f32> = vertices
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, e)| *e)
            .collect();

        let max = (
            xs.iter().cloned().fold(f32::NAN, f32::max),
            ys.iter().cloned().fold(f32::NAN, f32::max),
        );
        let min = (
            xs.iter().cloned().fold(f32::NAN, f32::min),
            ys.iter().cloned().fold(f32::NAN, f32::min),
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
}
