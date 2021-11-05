mod attribs;
mod buffers;
mod gl_program;
mod gl_utils;
mod shaders;
mod shape;
mod square;
mod utils;

use wasm_bindgen::prelude::*;

// use gl_program::GlProgram;
// use shaders::compile_shaders;
// use web_sys::WebGl2RenderingContext;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
pub fn hey() {
    alert("Hello there");
}

// #[wasm_bindgen]
// pub fn start(ctx: &WebGl2RenderingContext) {
//     console_log!("Something");

//     let prog = GlProgram::new(&ctx);
//     let locations = prog.get_a_locations();
//     console_log!("{:?}", locations);
//     console_log!("compiled shadeers");

//     // const vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

//     // let pos_attrib_loc = prog.get_attrib_location(&ShaderConstant::AVertexPosition);

//     // unsafe {
//     //     let pos_arr_buff_view = js_sys::Float32Array::view(&vertices);
//     //     ctx.buffer_data_with_array_buffer_view(
//     //         WebGl2RenderingContext::ARRAY_BUFFER,
//     //         &pos_arr_buff_view,
//     //         WebGl2RenderingContext::STATIC_DRAW,
//     //     );
//     // }

//     // let vao = ctx
//     //     .create_vertex_array()
//     //     .ok_or("Could not create vertex arr obj")
//     //     .unwrap();
//     // ctx.bind_vertex_array(Some(&vao));

//     // ctx.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
//     // ctx.enable_vertex_attrib_array(pos_attrib_loc as u32);

//     // ctx.bind_vertex_array(Some(&vao));

//     // let vert_count = (vertices.len() / 3) as i32;

//     // prog.draw(vert_count);
// }
