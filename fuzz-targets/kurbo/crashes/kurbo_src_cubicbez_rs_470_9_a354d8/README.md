# kurbo/src/cubicbez.rs:470:9

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/cubicbez.rs:470:9`
- last API call: `kurbo::BezPath::winding`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15187370725578541758

## Message

```
assertion failed: start.y.min(end.y) <= y && y <= start.y.max(end.y)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::CircleSegment;
use kurbo::Ellipse;
use kurbo::Point;

fn main() {
    let ellipse = Ellipse::default();
    let f64_v = ellipse.rotation();
    let _ = <kurbo::BezPath as kurbo::Shape>::winding(&<kurbo::Ellipse as kurbo::Shape>::into_path(ellipse, 1.0f64), <kurbo::Arc as kurbo::ParamCurve>::eval(&CircleSegment { center: Point::from((0.0f32, 0.0f32)), outer_radius: 0.0f64, inner_radius: f64::INFINITY, start_angle: f64_v, sweep_angle: 0.0f64 }.inner_arc(), 0.0f64));
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_cubicbez_rs_470_9_a354d8`).

## Output

```
thread 'main' (710632) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/cubicbez.rs:470:9:
assertion failed: start.y.min(end.y) <= y && y <= start.y.max(end.y)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <kurbo::cubicbez::CubicBez>::solve_monotonic_for_y
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/cubicbez.rs:470:9
   4: <kurbo::bezpath::PathSeg>::winding_inner
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:1143:31
   5: <kurbo::bezpath::PathSeg>::winding::{closure#0}
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:1156:49
   6: core::iter::adapters::map::map_fold::<core::ops::range::Range<f64>, i32, i32, <kurbo::bezpath::PathSeg>::winding::{closure#0}, <i32 as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<arrayvec::arrayvec::IntoIter<core::ops::range::Range<f64>, 5>, <kurbo::bezpath::PathSeg>::w... (+48 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
   7: <arrayvec::arrayvec::IntoIter<core::ops::range::Range<f64>, 5> as core::iter::traits::iterator::Iterator>::fold::<i32, core::iter::adapters::map::map_fold<core::ops::range::Range<f64>, i32, i32, <kurbo::bezpath::PathSeg>::winding::{closure#0}, <i32 as core::iter::traits::accum::Sum>::sum<core:... (+166 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2667:21
   8: <core::iter::adapters::map::Map<arrayvec::arrayvec::IntoIter<core::ops::range::Range<f64>, 5>, <kurbo::bezpath::PathSeg>::winding::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<i32, <i32 as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<arrayvec::arrayvec... (+108 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   9: <i32 as core::iter::traits::accum::Sum>::sum::<core::iter::adapters::map::Map<arrayvec::arrayvec::IntoIter<core::ops::range::Range<f64>, 5>, <kurbo::bezpath::PathSeg>::winding::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:52:22
  10: <core::iter::adapters::map::Map<arrayvec::arrayvec::IntoIter<core::ops::range::Range<f64>, 5>, <kurbo::bezpath::PathSeg>::winding::{closure#0}> as core::iter::traits::iterator::Iterator>::sum::<i32>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3670:9
  11: <kurbo::bezpath::PathSeg>::winding
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:1157:14
  12: <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::winding::{closure#0}
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:949:28
  13: core::iter::adapters::map::map_fold::<kurbo::bezpath::PathSeg, i32, i32, <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::winding::{closure#0}, <i32 as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<kurbo::b... (+269 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
  14: <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>> as core::iter::traits::iterator::Iterator>::fold::<i32, core::iter::adapters::map::map_fold<kurbo::bezpath::PathSeg, i32, i32, <kurbo::bezpath::Segments<core::iter::adapters::copied... (+437 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2667:21
  15: <core::iter::adapters::map::Map<kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>, <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::winding::{closure#0}> as core::ite... (+384 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
  16: <i32 as core::iter::traits::accum::Sum>::sum::<core::iter::adapters::map::Map<kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>, <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::... (+34 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:52:22
  17: <core::iter::adapters::map::Map<kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>, <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::winding::{closure#0}> as core::ite... (+42 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3670:9
  18: <kurbo::bezpath::Segments<core::iter::adapters::copied::Copied<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::winding
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:949:40
  19: <&[kurbo::bezpath::PathEl] as kurbo::shape::Shape>::winding
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:1525:40
  20: <kurbo::bezpath::BezPath as kurbo::shape::Shape>::winding
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:1445:25
  21: repro_kurbo_src_cubicbez_rs_470_9_a354d8::main
             at ./src/main.rs:8:13
  22: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
