# kurbo/src/fit.rs:646:61

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/fit.rs:646:61`
- last API call: `kurbo::fit_to_bezpath_opt`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 5641708333860798800

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Line;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Vec2;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let point = Point::from((0.0f64, 0.0f64));
    let f64_v = <kurbo::Line as kurbo::ParamCurveArclen>::arclen(&Line::from((Point { x: 0.0f64, y: 0.0f64 }, Vec2 { x: 0.0f64, y: 0.0f64 })), 0.0f64);
    let _ = kurbo::fit_to_bezpath_opt(&SimplifyBezPath::new(vec![PathEl::QuadTo(point, Point::new(f64_v, 1.0f64))]), 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_fit_rs_646_61_75818e`).

## Output

```
thread 'main' (465248) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/fit.rs:646:61:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<(kurbo::cubicbez::CubicBez, f64)>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: kurbo::fit::fit_to_bezpath_opt_inner::<kurbo::simplify::SimplifyBezPath>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/fit.rs:646:61
   6: kurbo::fit::fit_to_bezpath_opt::<kurbo::simplify::SimplifyBezPath>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/fit.rs:570:15
   7: repro_kurbo_src_fit_rs_646_61_75818e::main
             at ./src/main.rs:10:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
