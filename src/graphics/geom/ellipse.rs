use crate::graphics::shape::Drawing;
use super::Geom;
use std::f32::consts::PI;
use web_sys::WebGl2RenderingContext;
pub struct Ellipse {
    pub width: f32,
    pub height: f32,
    pub color: Vec<f32>,
}
pub struct Circle {
    pub radius: f32,
    pub color: Vec<f32>
}

impl Drawing for Ellipse {
    fn draw_shape(&self) -> Geom {
        let vertex_count: i32 = 400;
        let k = 2.0 * PI / 180.0 as f32;

        let vertices = (1..vertex_count / 2).fold(vec![], |acc, x| {
            acc.iter()
                .copied()
                .chain([
                    self.width * (k * x as f32).cos(),
                    self.height * (k * x as f32).sin(),
                    0.0,
                    0.0,
                ])
                .collect()
        });

        let color = self
            .color
            .iter()
            .cycle()
            .take(self.color.len() * vertex_count as usize)
            .map(|f| f.clone())
            .collect();

        Geom {
            vertices,
            color,
            vertex_count,
            mode: WebGl2RenderingContext::TRIANGLE_STRIP,
        }
    }
}

impl Drawing for Circle {
    fn draw_shape(&self) -> Geom {
        Ellipse {
            width: self.radius,
            height: self.radius,
            color: self.color.clone(),
        }
        .draw_shape()
    }
}
