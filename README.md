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

## Web (wasm)

The renderer also builds for the browser — no wasm-bindgen or npm, just a
`cdylib` blitted to a canvas by `web/index.html`:

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown --lib
cp target/wasm32-unknown-unknown/release/cube.wasm web/
python3 -m http.server -d web
```

Then open <http://localhost:8000>. Click the canvas for fullscreen.
(`--lib` matters: without it cargo also builds the native binary, which
doesn't exist for wasm.)

## Layout

- `src/lib.rs` — cube data, render loop, shading, culling
- `src/main.rs` — native front end (minifb window)
- `src/wasm.rs` — wasm exports (`web/index.html` drives them)
- `src/math.rs` — vector ops and rotation
- `src/raster.rs` — convex quad rasterizer

## Test

```
cargo test
```

Checks that face normals point outward, that at most 3 faces survive the
perspective cull at any rotation, and that the rasterizer fills quads.
