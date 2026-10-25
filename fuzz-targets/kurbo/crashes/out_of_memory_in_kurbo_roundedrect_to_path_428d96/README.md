# out of memory in kurbo::RoundedRect::to_path

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: out of memory
- last API call: `kurbo::RoundedRect::to_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 1976423772580277592

## Message

```
process ended abnormally (out of memory) while calling kurbo::RoundedRect::to_path
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
use kurbo::Axis;
use kurbo::LineIntersection;
use kurbo::MAX_EXTREMA;
use kurbo::Moments;
use kurbo::QuadSpline;
use kurbo::Rect;
use kurbo::RoundedRectRadii;
use kurbo::StrokeCtx;
use kurbo::TranslateScale;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Vertical;
    let _ = axis.pack_size(255.0f64, f64::MAX);
    let _ = TranslateScale::translate(Vec2::ZERO);
    let stroke_ctx = StrokeCtx::default();
    let _ = stroke_ctx.output();
    let rounded_rect_radii = RoundedRectRadii { top_left: f64::INFINITY, top_right: f64::NEG_INFINITY, bottom_right: 0.9999999999999999f64, bottom_left: 0.283f64 };
    let _ = rounded_rect_radii.clamp(10000000000.0f64);
    let _ = <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(<kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments { moment_x: 0.478f64, moment_y: 0.573f64, moment_xx: -1e300f64, moment_xy: 557.0f64, moment_yy: f64::MAX }, Moments::default()), <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments::default(), <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments::default(), Moments::default())));
    let _ = LineIntersection { line_t: 0.266f64, segment_t: -69.6f64 };
    let stroke_ctx2 = StrokeCtx::default();
    let _ = stroke_ctx2.output();
    let affine = Affine::skew(f64::MIN, 71.8f64);
    let affine2 = affine.pre_skew(-1e300f64, -80.9f64);
    let _ = affine2.inverse();
    let quad_spline = QuadSpline::new(vec![]);
    let _ = quad_spline.points();
    let _ = <kurbo::RoundedRect as kurbo::Shape>::to_path(&Rect::new(-4.9786148735079455e270f64, 9007199254740992.0f64, 1e300f64, f64::MIN).inflate(22.5f64, 537.0f64).to_rounded_rect(rounded_rect_radii), 65.7f64);
    let _ = MAX_EXTREMA;
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_kurbo_roundedrect_to_path_428d96`).

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
   9: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::extend_desugared::<kurbo::rounded_rect::RoundedRectPathIter>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4104:22
  10: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_extend::SpecExtend<kurbo::bezpath::PathEl, kurbo::rounded_rect::RoundedRectPathIter>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:18:14
  11: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<kurbo::bezpath::PathEl, kurbo::rounded_rect::RoundedRectPathIter>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:41:9
  12: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_from_iter::SpecFromIter<kurbo::bezpath::PathEl, kurbo::rounded_rect::RoundedRectPathIter>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  13: <alloc::vec::Vec<kurbo::bezpath::PathEl> as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<kurbo::rounded_rect::RoundedRectPathIter>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  14: <kurbo::rounded_rect::RoundedRectPathIter as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  15: <kurbo::bezpath::BezPath as core::iter::traits::collect::FromIterator<kurbo::bezpath::PathEl>>::from_iter::<kurbo::rounded_rect::RoundedRectPathIter>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:509:47
  16: <kurbo::rounded_rect::RoundedRectPathIter as core::iter::traits::iterator::Iterator>::collect::<kurbo::bezpath::BezPath>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  17: <kurbo::rounded_rect::RoundedRect as kurbo::shape::Shape>::to_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/shape.rs:68:39
  18: repro_out_of_memory_in_kurbo_roundedrect_to_path_428d96::main
             at ./src/main.rs:30:13
  19: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
