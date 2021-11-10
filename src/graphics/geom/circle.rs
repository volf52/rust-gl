// use super::Geom;
// use std::f32::consts::PI;

// use wasm_bindgen::prelude::*;
// use web_sys::WebGl2RenderingContext;

// #[wasm_bindgen]
// impl Geom {
//     pub fn new_circle(radius: f32) -> Self {
//         let tuplevert = (1..100).map(|f| {
//             (
//                 radius * (2.0 * f as f32 * PI / 100.0).cos(),
//                 radius * (2.0 * f as f32 * PI / 100.0).sin(),
//             )
//         });

//         let vertices = tuplevert.map(|f| f.0);
//     }
// }
