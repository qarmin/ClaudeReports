# timeout in kurbo::CircleSegment::into_path

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::CircleSegment::into_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 13586316498447270429

## Message

```
process ended abnormally (timeout) while calling kurbo::CircleSegment::into_path
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
use kurbo::Cap;
use kurbo::CircleSegment;
use kurbo::DEFAULT_ACCURACY;
use kurbo::Diagonal2;
use kurbo::LineIntersection;
use kurbo::Moments;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::StrokeCtx;
use kurbo::SvgArc;
use kurbo::SvgParseError;
use kurbo::Vec2;
use kurbo::common::GAUSS_LEGENDRE_COEFFS_8_HALF;
use kurbo::simplify::SimplifyOptLevel;

fn main() {
    let svg_parse_error = SvgParseError::Wrong;
    let bez_path = <kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(<kurbo::Diagonal2 as ::core::ops::Mul<kurbo::Point>>::mul(Diagonal2::new(f64::INFINITY, 1.0f64), Point { x: 0.241f64, y: 0.0f64 }), 9007199254740992.0f64, 0.836f64, 0.5f64, -1.4174536431560663e60f64), 10000000000.0f64);
    let _ = <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments { moment_x: -1e300f64, moment_y: 1.0f64, moment_xx: 0.5f64, moment_xy: 1.0f64, moment_yy: f64::MAX }, Moments::default());
    let f64_v = DEFAULT_ACCURACY;
    let _ = LineIntersection { line_t: -0.0f64, segment_t: 308.0f64 };
    let diagonal2 = Diagonal2 { xx: 0.308f64, yy: 0.182f64 };
    let diagonal22 = diagonal2.abs();
    let vec2 = diagonal22.scale_normal(Vec2::from_angle(f64_v));
    let point = Point::from((8.3f64, 1e-310f64));
    let stroke_ctx = StrokeCtx::default();
    let _ = stroke_ctx.output();
    let _ = Cap::Round;
    let _ = SimplifyOptLevel::Subdivide;
    let _ = <kurbo::PathSeg as kurbo::ParamCurveNearest>::nearest(&PathSeg::from(<kurbo::BezPath as kurbo::Shape>::as_line(&bez_path).unwrap()), point, -1.0f64);
    let f64_v2 = vec2.length();
    let _ = GAUSS_LEGENDRE_COEFFS_8_HALF.to_vec();
    let _ = <kurbo::Circle as kurbo::Shape>::as_circle(&<kurbo::Rect as kurbo::Shape>::as_circle(&<kurbo::Arc as kurbo::Shape>::as_rect(&Arc::from_svg_arc(&SvgArc { from: point, to: point, radii: vec2, x_rotation: f64_v2, large_arc: false, sweep: true }).unwrap()).unwrap()).unwrap());
    let _ = svg_parse_error.to_string();
    let _ = SvgParseError::UninitializedPath;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_circlesegment_into_path_86d55c`).

## Output

```
memory allocation of 3758096384 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::extend_desugared::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kur... (+136 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4104:22
  10: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_extend::SpecExtend<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::Pa... (+192 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:18:14
  11: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<... (+208 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:41:9
  12: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter::SpecFromIter<kurbo::bezpath::PathEl, core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpat... (+195 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  13: <alloc::vec::Vec<kurbo::bezpath::PathEl> as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Onc... (+198 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  14: <core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter... (+169 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  15: <kurbo::bezpath::BezPath as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath... (+182 chars)
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:509:47
  16: <core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<kurbo::bezpath::PathEl>, core::iter::sources::once::Once<kurbo::bezpath::PathEl>>, kurbo::arc::ArcAppendIter>, core::iter... (+153 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  17: <kurbo::circle::CircleSegment as kurbo::shape::Shape>::to_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/shape.rs:68:39
  18: <kurbo::circle::CircleSegment as kurbo::shape::Shape>::into_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/shape.rs:83:14
  19: repro_timeout_in_kurbo_circlesegment_into_path_86d55c::main
             at ./src/main.rs:19:20
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
