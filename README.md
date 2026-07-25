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

## Prebuilt binaries

When the `version` in `Cargo.toml` is bumped on `master`, CI publishes a
[GitHub Release](https://github.com/lra/3dcube/releases) with archives for:

- Linux x86_64 and ARM64
- macOS ARM64 and x86_64
- Windows x86_64 and ARM64
- Web: `cube-<version>-web.zip` with `index.html` + `cube.wasm`, ready to
  serve from any static host

Each native archive is a single `cube` binary (`.exe` on Windows). Release
notes are generated automatically from commits and pull requests since the
previous tag.

To cut a release: bump `version` in `Cargo.toml` (and commit the lockfile if
dependencies changed), merge to `master`. CI tags `v<version>` and uploads the
archives once that tag does not already exist.

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
