# epaint/src/shapes/bezier_shape.rs:141:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/bezier_shape.rs:141:9`
- last API call: `epaint::CubicBezierShape::split_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7875049757093032636

## Message

```
range should be in [0.0,1.0]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use emath::Pos2;
use epaint::ColorMode;
use epaint::CubicBezierShape;
use epaint::FontColorTransferFunction;
use epaint::PathStroke;
use epaint::StrokeKind;

fn main() {
    let pos2 = Pos2::default();
    let cubic_bezier_shape = CubicBezierShape { points: [pos2, pos2, pos2, pos2], closed: false, fill: FontColorTransferFunction::LIGHT_MODE_DEFAULT.color_from_coverage(0.0f32), stroke: PathStroke { width: 0.0f32, color: ColorMode::TRANSPARENT, kind: StrokeKind::Inside } };
    let _ = cubic_bezier_shape.split_range(0.0f32..256.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_bezier_shape_rs_141_9_414d32`).

## Output

```
thread 'main' (193848) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:141:9:
range should be in [0.0,1.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::shapes::bezier_shape::CubicBezierShape>::split_range
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:141:9
   3: repro_epaint_src_shapes_bezier_shape_rs_141_9_414d32::main
             at ./src/main.rs:11:32
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
