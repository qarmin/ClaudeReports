# epaint/src/shapes/bezier_shape.rs:454:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/bezier_shape.rs:454:9`
- last API call: `epaint::QuadraticBezierShape::sample`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4539626167992847105

## Message

```
the sample value should be in [0.0,1.0]
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
use emath::Align2;
use emath::Pos2;
use emath::Rect;
use epaint::PathStroke;
use epaint::QuadraticBezierShape;

fn main() {
    let align2 = Align2::RIGHT_TOP;
    let quadratic_bezier_shape = QuadraticBezierShape { points: [Pos2 { x: 0.0f32, y: 0.0f32 }, align2.pos_in_rect(&Rect::everything_above(0.0f32)), Pos2::new(0.0f32, 0.0f32)], closed: false, fill: Color32::ORANGE, stroke: PathStroke::default() };
    let _ = quadratic_bezier_shape.sample(10000000000.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_bezier_shape_rs_454_9_3306eb`).

## Output

```
thread 'main' (223584) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:454:9:
the sample value should be in [0.0,1.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::shapes::bezier_shape::QuadraticBezierShape>::sample
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:454:9
   3: repro_epaint_src_shapes_bezier_shape_rs_454_9_3306eb::main
             at ./src/main.rs:11:36
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
