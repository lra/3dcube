// Flat-shaded rotating cube in a window (minifb). Esc or close to quit.
// The renderer itself lives in lib.rs, shared with the wasm build (web/).

use cube::{H, W, render};
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut window = Window::new("cube", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut t = 0f32;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(t, &mut buf);
        window.update_with_buffer(&buf, W, H).unwrap();
        t += 0.02;
    }
}
