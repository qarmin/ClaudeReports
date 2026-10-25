# kurbo/src/bezpath.rs:223:9

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/bezpath.rs:223:9`
- last API call: `kurbo::BezPath::push`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8538147398349408523

## Message

```
BezPath must begin with MoveTo
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Axis;
use kurbo::BezPath;
use kurbo::PathEl;
use kurbo::Point;

fn main() {
    let mut bez_path = BezPath::with_capacity(0usize);
    bez_path.push(PathEl::QuadTo(Axis::Horizontal.pack_point(0.0f64, 0.0f64), Point::ZERO));
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_bezpath_rs_223_9_915335`).

## Output

```
thread 'main' (538910) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:223:9:
BezPath must begin with MoveTo
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <kurbo::bezpath::BezPath>::push
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:223:9
   3: repro_kurbo_src_bezpath_rs_223_9_915335::main
             at ./src/main.rs:8:14
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
