use web_sys::WebGl2RenderingContext;

use crate::{
    graphics::shapes::utils::calc_n_vertices,
    math::{bounds::BoundingDims, Matrix},
    textures::utils::{TextureGen, TextureOrColor},
};

#[derive(Debug, Clone)]
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
    ) -> Geom {
        let vertices = calc_n_vertices(width, height, no_sides);

        Geom::new(
            &vertices,
            Matrix::translation(x, y),
            WebGl2RenderingContext::TRIANGLE_FAN,
            no_sides as i32,
            color_or_texture,
        )
    }

    pub fn set_texture(&mut self, text_gen: &impl TextureGen) {
        self.texture_data = text_gen.to_enum();
    }

    pub fn rotate(&mut self, angle: f32) {
        self.u_mat = self.u_mat.rotate(angle);
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.u_mat.translate_inplace(tx, ty);
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.u_mat.scale_inplace(x, y);
    }

    pub fn get_dims(&self) -> BoundingDims {
        BoundingDims::from_vertices(&self.vertices)
    }

    pub fn calc_tex_coords(vertices: &[f32]) -> Vec<f32> {
        let ((min_x, max_x), (min_y, max_y)) = BoundingDims::get_width_height_range(vertices);

        let width = max_x - min_x;
        let height = max_y - min_y;

        vertices
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                if idx % 2 == 0 {
                    (p - min_y) / height // normalize y
                } else {
                    (p - min_x) / width // normalize x
                }
            })
            .collect()
    }
}
