use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub fn create_buffer_with_type(ctx: &WebGl2RenderingContext, type_: u32) -> WebGlBuffer {
    let buffer = ctx
        .create_buffer()
        .ok_or("Failed to create buffer")
        .unwrap();

    ctx.bind_buffer(type_, Some(&buffer));

    buffer
}

pub fn create_array_buffer(ctx: &WebGl2RenderingContext) -> WebGlBuffer {
    create_buffer_with_type(ctx, WebGl2RenderingContext::ARRAY_BUFFER)
}

pub fn bind_f32_buffer_data(ctx: &WebGl2RenderingContext, data: &Vec<f32>) {
    unsafe {
        let view = js_sys::Float32Array::view(data.as_slice());
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    };
}

pub fn bind_u8_buffer_data(ctx: &WebGl2RenderingContext, data: &Vec<u8>) {
    unsafe {
        let view = js_sys::Uint8Array::view(data.as_slice());
        ctx.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    };
}
