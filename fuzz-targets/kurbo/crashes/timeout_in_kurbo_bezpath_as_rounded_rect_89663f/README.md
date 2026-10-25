# timeout in kurbo::BezPath::as_rounded_rect

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::as_rounded_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3066909630539194436

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::as_rounded_rect
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Affine;
use kurbo::Arc;
use kurbo::Cap;
use kurbo::CircleSegment;
use kurbo::CuspType;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::LineIntersection;
use kurbo::MAX_EXTREMA;
use kurbo::PathEl;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::Rect;
use kurbo::RoundedRect;
use kurbo::RoundedRectRadii;
use kurbo::Stroke;
use kurbo::StrokeCtx;
use kurbo::Vec2;

fn main() {
    let _ = PathEl::MoveTo(RoundedRect::default().origin());
    let line_intersection = LineIntersection { line_t: 10000000000.0f64, segment_t: f64::MAX };
    let _ = line_intersection.is_finite();
    let _ = MAX_EXTREMA;
    let rect = Rect::new(1.0f64, 0.0f64, 207249957868.58365f64, f64::MAX);
    let ellipse = rect.to_ellipse();
    let point = ellipse.center();
    let _ = CuspType::Loop;
    let _ = CuspType::DoubleInflection;
    let _ = Stroke::default();
    let rounded_rect_radii = RoundedRectRadii::from((-35.8f64, -33.0f64, -40.4f64, 7.954071908978831e29f64));
    let _ = StrokeCtx::default();
    let affine = Affine::new([0.991f64, 81.0f64, 10000000000.0f64, 6.36503106460998e-230f64, 10000000000.0f64, 577.0f64]);
    let affine2 = affine.pre_scale_non_uniform(-0.0f64, -0.0f64);
    let _ = affine2.transform_rect_bbox(rect.expand());
    let rounded_rect = <kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&<kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(point, -1e300f64, f64::NEG_INFINITY, 173.0f64, 0.222f64), 2.2250738585072014e-308f64)).unwrap();
    let _ = <kurbo::PathSeg as kurbo::ParamCurve>::subdivide(&PathSeg::Line(<kurbo::RoundedRect as kurbo::Shape>::as_line(&rounded_rect).unwrap()));
    let _ = Diagonal2::new(1e-310f64, 1.1946697462982718e140f64);
    let _ = rounded_rect_radii.as_single_radius();
    let affine3 = affine2.pre_scale_non_uniform(50.5f64, f64::MIN);
    let _ = affine3.with_translation(ellipse.with_center(Point::from((1.0000001f32, f32::NEG_INFINITY))).radii().rotate_scale(Vec2::new(34.8f64, 340.0f64)));
    let _ = Cap::Butt;
    let _ = Join::Bevel;
    let bez_path = <kurbo::BezPath as kurbo::Shape>::into_path(<kurbo::Arc as kurbo::Shape>::into_path(Arc { center: point, radii: Vec2 { x: f64::MAX, y: 2.2250738585072014e-308f64 }, start_angle: 5.9068586614308054e-229f64, sweep_angle: 2.2250738585072014e-308f64, x_rotation: f64::NAN }, 0.0f64), -1e300f64);
    let _ = bez_path.is_finite();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_as_rounded_rect_89663f`).

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
  19: repro_timeout_in_kurbo_bezpath_as_rounded_rect_89663f::main
             at ./src/main.rs:36:75
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
