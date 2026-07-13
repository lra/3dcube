# 3dcube

Classic demoscene flat-shaded rotating cube, software-rendered in Rust. Six
uniquely tinted blue faces, brightness per face = ambient + `dot(normal, light)`
(diffuse shading), perspective projection, backface culling. Only dependency is
[minifb](https://crates.io/crates/minifb) for the window and pixel buffer.

## Run

```
cargo run --release
```

Esc or close the window to quit.

## Layout

- `src/main.rs` — cube data, render loop, shading, culling
- `src/math.rs` — vector ops and rotation
- `src/raster.rs` — convex quad rasterizer

## Test

```
cargo test
```

Checks that face normals point outward, that at most 3 faces survive the
perspective cull at any rotation, and that the rasterizer fills quads.
