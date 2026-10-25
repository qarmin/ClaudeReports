# kurbo/src/bezpath.rs:275:9

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/bezpath.rs:275:9`
- last API call: `kurbo::simplify::simplify_bezpath`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 707137938489030857

## Message

```
uninitialized subpath (missing MoveTo)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::PathEl;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let _ = kurbo::simplify::simplify_bezpath(vec![PathEl::ClosePath], 0.0f64, &SimplifyOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_bezpath_rs_275_9_5a4815`).

## Output

```
thread 'main' (2156435) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:275:9:
uninitialized subpath (missing MoveTo)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <kurbo::bezpath::BezPath>::close_path
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:275:9
   3: kurbo::simplify::simplify_bezpath::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:337:30
   4: repro_kurbo_src_bezpath_rs_275_9_5a4815::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
