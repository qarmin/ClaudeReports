# epaint/src/shapes/bezier_shape.rs:178:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/bezier_shape.rs:178:9`
- last API call: `epaint::CubicBezierShape::to_path_shapes`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2126668659754492759

## Message

```
the tolerance should be positive
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use emath::Pos2;
use epaint::CubicBezierShape;
use epaint::PathStroke;

fn main() {
    let cubic_bezier_shape = CubicBezierShape::from(CubicBezierShape::from_points_stroke([Pos2::from((0.0f32, 0.0f32)), Pos2::ZERO, Pos2::from([1.0f32, 1.0f32]), Pos2::default()], false, Color32::BROWN, PathStroke::default()));
    let _ = cubic_bezier_shape.to_path_shapes(None, None);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_bezier_shape_rs_178_9_e65cbb`).

## Output

```
thread 'main' (191801) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:178:9:
the tolerance should be positive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::shapes::bezier_shape::CubicBezierShape>::num_quadratics
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:178:9
   3: epaint::shapes::bezier_shape::flatten_cubic_bezier_with_t::<<epaint::shapes::bezier_shape::CubicBezierShape>::flatten_closed::{closure#3}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:530:32
   4: <epaint::shapes::bezier_shape::CubicBezierShape>::for_each_flattened_with_t::<<epaint::shapes::bezier_shape::CubicBezierShape>::flatten_closed::{closure#3}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:317:9
   5: <epaint::shapes::bezier_shape::CubicBezierShape>::flatten_closed
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:302:22
   6: <epaint::shapes::bezier_shape::CubicBezierShape>::to_path_shapes
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:63:35
   7: repro_epaint_src_shapes_bezier_shape_rs_178_9_e65cbb::main
             at ./src/main.rs:8:32
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
