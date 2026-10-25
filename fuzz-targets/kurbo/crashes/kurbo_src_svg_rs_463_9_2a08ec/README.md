# kurbo/src/svg.rs:463:9

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/svg.rs:463:9`
- last API call: `kurbo::Arc::from_svg_arc`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18432457804069013413

## Message

```
assertion failed: sum_of_sq != 0.0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Arc;
use kurbo::Axis;
use kurbo::LineIntersection;
use kurbo::RoundedRect;
use kurbo::SvgArc;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Vertical;
    let point = axis.pack_point(0.0f64, 2.2250738585072014e-308f64);
    let line_intersection = LineIntersection { line_t: 0.0f64, segment_t: 0.0f64 };
    let bool_v = line_intersection.is_nan();
    let _ = Arc::from_svg_arc(&SvgArc { from: RoundedRect::default().origin(), to: point, radii: Vec2::from_angle(1.0f64), x_rotation: 0.0f64, large_arc: bool_v, sweep: false });
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_svg_rs_463_9_2a08ec`).

## Output

```
thread 'main' (181219) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/svg.rs:463:9:
assertion failed: sum_of_sq != 0.0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <kurbo::arc::Arc>::from_svg_arc
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/svg.rs:463:9
   4: repro_kurbo_src_svg_rs_463_9_2a08ec::main
             at ./src/main.rs:13:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
