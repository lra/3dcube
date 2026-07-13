// Flat-shaded rotating cube in a window (minifb). Esc or close to quit.
mod math;
mod raster;

use math::{cross, dot, rot, sub, V3};
use minifb::{Key, Window, WindowOptions};
use raster::{fill_quad, H, W};

#[rustfmt::skip]
const VERTS: [V3; 8] = [
    [-1., -1., -1.], [1., -1., -1.], [1., 1., -1.], [-1., 1., -1.],
    [-1., -1.,  1.], [1., -1.,  1.], [1., 1.,  1.], [-1., 1.,  1.],
];
// CCW seen from outside
const FACES: [[usize; 4]; 6] = [
    [0, 3, 2, 1], // -z
    [4, 5, 6, 7], // +z
    [0, 4, 7, 3], // -x
    [1, 2, 6, 5], // +x
    [0, 1, 5, 4], // -y
    [3, 7, 6, 2], // +y
];
// six distinct blues
const BASE: [[f32; 3]; 6] = [
    [70., 90., 255.],
    [30., 140., 255.],
    [0., 60., 220.],
    [100., 160., 255.],
    [50., 50., 200.],
    [0., 110., 235.],
];
const CAMERA: V3 = [0., 0., -4.];

fn main() {
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
        let rv: Vec<V3> = VERTS.iter().map(|&p| rot(p, t * 0.6, t)).collect();
        let proj: Vec<[f32; 2]> = rv
            .iter()
            .map(|p| {
                let s = 380.0 / (p[2] - CAMERA[2]);
                [W as f32 / 2.0 + p[0] * s, H as f32 / 2.0 + p[1] * s]
            })
            .collect();

        for (f, col) in FACES.iter().zip(&BASE) {
            let n = cross(sub(rv[f[1]], rv[f[0]]), sub(rv[f[3]], rv[f[0]]));
            // backface cull against the real view ray, not infinity on -z
            if dot(n, sub(rv[f[0]], CAMERA)) >= 0.0 {
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
        for f in FACES {
            let n = cross(sub(VERTS[f[1]], VERTS[f[0]]), sub(VERTS[f[3]], VERTS[f[0]]));
            let center = f.iter().fold([0.; 3], |a, &i| {
                [a[0] + VERTS[i][0], a[1] + VERTS[i][1], a[2] + VERTS[i][2]]
            });
            assert!(dot(n, center) > 0.0, "face {f:?} normal points inward");
        }
        // perspective cull: never more than 3 visible faces, any rotation
        // (the old n.z-at-infinity cull let a 4th face through near silhouettes)
        for i in 0..500 {
            let t = i as f32 * 0.037;
            let rv: Vec<V3> = VERTS.iter().map(|&p| rot(p, t * 0.6, t)).collect();
            let visible = FACES
                .iter()
                .filter(|f| {
                    let n = cross(sub(rv[f[1]], rv[f[0]]), sub(rv[f[3]], rv[f[0]]));
                    dot(n, sub(rv[f[0]], CAMERA)) < 0.0
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
