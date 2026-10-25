# epaint/src/shapes/bezier_shape.rs:229:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/bezier_shape.rs:229:9`
- last API call: `epaint::CubicBezierShape::sample`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 11240432138454092972

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
use emath::Pos2;
use epaint::CubicBezierShape;
use epaint::PathStroke;

fn main() {
    let pos2 = Pos2::default();
    let cubic_bezier_shape = CubicBezierShape { points: [Pos2 { x: 0.0f32, y: 0.0f32 }, pos2, pos2, pos2], closed: false, fill: Color32::from_black_alpha(0u8), stroke: PathStroke::default() };
    let _ = cubic_bezier_shape.sample(10000000000.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_bezier_shape_rs_229_9_8723ac`).

## Output

```
thread 'main' (293948) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:229:9:
the sample value should be in [0.0,1.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::shapes::bezier_shape::CubicBezierShape>::sample
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/bezier_shape.rs:229:9
   3: repro_epaint_src_shapes_bezier_shape_rs_229_9_8723ac::main
             at ./src/main.rs:9:32
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
