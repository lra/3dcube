pub const W: usize = 640;
pub const H: usize = 640;

// ponytail: per-pixel inside test over the bbox; convex quad, no scanline needed
pub fn fill_quad(q: &[[f32; 2]; 4], color: u32, buf: &mut [u32]) {
    let edge = |a: [f32; 2], b: [f32; 2], p: [f32; 2]| {
        (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
    };
    let area = edge(q[0], q[1], q[2]) + edge(q[0], q[2], q[3]);
    let (x0, x1) = q.iter().fold((W as f32, 0f32), |(lo, hi), p| (lo.min(p[0]), hi.max(p[0])));
    let (y0, y1) = q.iter().fold((H as f32, 0f32), |(lo, hi), p| (lo.min(p[1]), hi.max(p[1])));
    for y in y0.max(0.0) as usize..=y1.min(H as f32 - 1.0) as usize {
        for x in x0.max(0.0) as usize..=x1.min(W as f32 - 1.0) as usize {
            let p = [x as f32 + 0.5, y as f32 + 0.5];
            if (0..4).all(|i| edge(q[i], q[(i + 1) % 4], p) * area >= 0.0) {
                buf[y * W + x] = color;
            }
        }
    }
}
