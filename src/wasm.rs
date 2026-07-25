// Plain `extern "C"` exports — no wasm-bindgen, no npm. web/index.html
// instantiates the module directly and blits the RGBA buffer to a canvas.

use crate::{H, W, render};

pub struct WasmCube {
    t: f32,
    buf: Vec<u32>,
    rgba: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn cube_new() -> *mut WasmCube {
    Box::into_raw(Box::new(WasmCube {
        t: 0.0,
        buf: vec![0; W * H],
        rgba: vec![0; W * H * 4],
    }))
}

/// Advance one frame; returns a pointer to W*H*4 RGBA bytes in wasm memory.
#[unsafe(no_mangle)]
pub extern "C" fn cube_frame(p: *mut WasmCube) -> *const u8 {
    let c = unsafe { &mut *p };
    render(c.t, &mut c.buf);
    c.t += 0.02;
    for (px, out) in c.buf.iter().zip(c.rgba.chunks_exact_mut(4)) {
        out[0] = (px >> 16) as u8;
        out[1] = (px >> 8) as u8;
        out[2] = *px as u8;
        out[3] = 255;
    }
    c.rgba.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn cube_width() -> usize {
    W
}

#[unsafe(no_mangle)]
pub extern "C" fn cube_height() -> usize {
    H
}
