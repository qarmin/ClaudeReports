# kurbo/src/simplify.rs:322:36

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/simplify.rs:322:36`
- last API call: `kurbo::simplify::simplify_bezpath`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6765339055161951000

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
use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Vec2;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let diagonal2 = Diagonal2::new(0.0f64, 0.0f64);
    let vec2 = diagonal2.scale_normal(Vec2::new(0.0f64, 0.0f64));
    let point = <kurbo::Arc as kurbo::ParamCurve>::end(&Arc { center: Point::ORIGIN, radii: vec2, start_angle: 0.0f64, sweep_angle: 0.0f64, x_rotation: 0.0f64 });
    let path_el = PathEl::QuadTo(point.expand(), point);
    let _ = kurbo::simplify::simplify_bezpath(vec![path_el], 0.0f64, &SimplifyOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_simplify_rs_322_36_2ad12e`).

## Output

```
thread 'main' (437826) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:322:36:
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
   4: <core::option::Option<kurbo::point::Point>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: kurbo::simplify::simplify_bezpath::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:322:36
   6: repro_kurbo_src_simplify_rs_322_36_2ad12e::main
             at ./src/main.rs:13:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
