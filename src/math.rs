pub type V3 = [f32; 3];

pub fn rot(p: V3, ax: f32, ay: f32) -> V3 {
    let (sx, cx) = ax.sin_cos();
    let (sy, cy) = ay.sin_cos();
    let [x, y, z] = p;
    let (y, z) = (y * cx - z * sx, y * sx + z * cx); // around X
    let (x, z) = (x * cy + z * sy, -x * sy + z * cy); // around Y
    [x, y, z]
}

pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
