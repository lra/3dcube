// Flat-shaded rotating cube in a window (minifb). Esc or close to quit.
use minifb::{Key, Window, WindowOptions};

const W: usize = 640;
const H: usize = 640;

type V3 = [f32; 3];

fn rot(p: V3, ax: f32, ay: f32) -> V3 {
    let (sx, cx) = ax.sin_cos();
    let (sy, cy) = ay.sin_cos();
    let [x, y, z] = p;
    let (y, z) = (y * cx - z * sx, y * sx + z * cx); // around X
    let (x, z) = (x * cy + z * sy, -x * sy + z * cy); // around Y
    [x, y, z]
}

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

// ponytail: per-pixel inside test over the bbox; convex quad, no scanline needed
fn fill_quad(q: &[[f32; 2]; 4], color: u32, buf: &mut [u32]) {
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

fn main() {
    #[rustfmt::skip]
    let verts: [V3; 8] = [
        [-1., -1., -1.], [1., -1., -1.], [1., 1., -1.], [-1., 1., -1.],
        [-1., -1.,  1.], [1., -1.,  1.], [1., 1.,  1.], [-1., 1.,  1.],
    ];
    // CCW seen from outside
    let faces: [[usize; 4]; 6] = [
        [0, 3, 2, 1], // -z
        [4, 5, 6, 7], // +z
        [0, 4, 7, 3], // -x
        [1, 2, 6, 5], // +x
        [0, 1, 5, 4], // -y
        [3, 7, 6, 2], // +y
    ];
    // six distinct blues
    let base: [[f32; 3]; 6] = [
        [70., 90., 255.],
        [30., 140., 255.],
        [0., 60., 220.],
        [100., 160., 255.],
        [50., 50., 200.],
        [0., 110., 235.],
    ];
    let light = {
        let v: V3 = [-0.5, -0.7, -0.6];
        let n = dot(v, v).sqrt();
        [v[0] / n, v[1] / n, v[2] / n]
    };

    let mut window = Window::new("cube", W, H, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    let mut buf = vec![0u32; W * H];
    let mut t = 0f32;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buf.fill(0);
        let rv: Vec<V3> = verts.iter().map(|&p| rot(p, t * 0.6, t)).collect();
        let proj: Vec<[f32; 2]> = rv
            .iter()
            .map(|p| {
                let s = 380.0 / (p[2] + 4.0);
                [W as f32 / 2.0 + p[0] * s, H as f32 / 2.0 + p[1] * s]
            })
            .collect();

        for (f, col) in faces.iter().zip(&base) {
            let n = cross(sub(rv[f[1]], rv[f[0]]), sub(rv[f[3]], rv[f[0]]));
            // backface cull against the real view ray: camera is at (0,0,-4)
            // (matches the perspective divisor z+4), not at infinity on -z
            let view = sub(rv[f[0]], [0., 0., -4.]);
            if dot(n, view) >= 0.0 {
                continue;
            }
            let shade = 0.2 + 0.8 * (dot(n, light) / dot(n, n).sqrt()).max(0.0);
            let color = ((col[0] * shade) as u32) << 16
                | ((col[1] * shade) as u32) << 8
                | (col[2] * shade) as u32;
            let q = [proj[f[0]], proj[f[1]], proj[f[2]], proj[f[3]]];
            fill_quad(&q, color, &mut buf);
        }

        window.update_with_buffer(&buf, W, H).unwrap();
        t += 0.02;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normals_point_outward_and_fill_hits_center() {
        // every face normal of the unit cube must point away from the origin
        let verts: [V3; 8] = [
            [-1., -1., -1.], [1., -1., -1.], [1., 1., -1.], [-1., 1., -1.],
            [-1., -1.,  1.], [1., -1.,  1.], [1., 1.,  1.], [-1., 1.,  1.],
        ];
        let faces: [[usize; 4]; 6] = [
            [0, 3, 2, 1], [4, 5, 6, 7], [0, 4, 7, 3],
            [1, 2, 6, 5], [0, 1, 5, 4], [3, 7, 6, 2],
        ];
        for f in faces {
            let n = cross(sub(verts[f[1]], verts[f[0]]), sub(verts[f[3]], verts[f[0]]));
            let center = f.iter().fold([0.; 3], |a, &i| {
                [a[0] + verts[i][0], a[1] + verts[i][1], a[2] + verts[i][2]]
            });
            assert!(dot(n, center) > 0.0, "face {f:?} normal points inward");
        }
        // perspective cull: never more than 3 visible faces, any rotation
        // (the old n.z-at-infinity cull let a 4th face through near silhouettes)
        for i in 0..500 {
            let t = i as f32 * 0.037;
            let rv: Vec<V3> = verts.iter().map(|&p| rot(p, t * 0.6, t)).collect();
            let visible = faces
                .iter()
                .filter(|f| {
                    let n = cross(sub(rv[f[1]], rv[f[0]]), sub(rv[f[3]], rv[f[0]]));
                    dot(n, sub(rv[f[0]], [0., 0., -4.])) < 0.0
                })
                .count();
            assert!(visible <= 3, "{visible} faces visible at t={t}");
        }
        // rasterizer fills the middle of an on-screen square
        let mut buf = vec![0u32; W * H];
        fill_quad(&[[10., 10.], [30., 10.], [30., 30.], [10., 30.]], 0xABCDEF, &mut buf);
        assert_eq!(buf[20 * W + 20], 0xABCDEF);
        assert_eq!(buf[5 * W + 5], 0);
    }
}
