# kurbo/src/bezpath.rs:208:9

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/bezpath.rs:208:9`
- last API call: `kurbo::BezPath::from_vec`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14317810086989197469

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
use kurbo::BezPath;
use kurbo::Rect;

fn main() {
    let _ = BezPath::from_vec(vec![<kurbo::Rect as kurbo::Shape>::to_path(&Rect::default(), 0.0f64).pop().unwrap()]);
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_bezpath_rs_208_9_6aae70`).

## Output

```
thread 'main' (378251) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:208:9:
BezPath must begin with MoveTo
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <kurbo::bezpath::BezPath>::from_vec
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:208:9
   3: repro_kurbo_src_bezpath_rs_208_9_6aae70::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
