# kurbo/src/simplify.rs:329:36

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/simplify.rs:329:36`
- last API call: `kurbo::simplify::simplify_bezpath`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12947106552494582280

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
use kurbo::CubicBez;
use kurbo::PathEl;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let cubic_bez = CubicBez { p0: Point::from((0.0f32, 0.0f32)), p1: Point::default(), p2: PathEl::MoveTo(Point::default()).end_point().unwrap(), p3: Point::default() };
    let _ = kurbo::simplify::simplify_bezpath(vec![PathSeg::from(cubic_bez).as_path_el()], 0.0f64, &SimplifyOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_simplify_rs_329_36_5a9b24`).

## Output

```
thread 'main' (428798) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:329:36:
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
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:329:36
   6: repro_kurbo_src_simplify_rs_329_36_5a9b24::main
             at ./src/main.rs:9:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
