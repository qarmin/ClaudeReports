# kurbo/src/common.rs:714:16

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/common.rs:714:16`
- last API call: `kurbo::common::solve_itp`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12777893022868112007

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, f64::NEG_INFINITY, -1.0f64, 1usize, 0.0f64, 0.0f64, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_common_rs_714_16_465a3a`).

## Output

```
thread 'main' (482468) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/common.rs:714:16:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: kurbo::common::solve_itp_fallible::<core::convert::Infallible, kurbo::common::solve_itp<repro_kurbo_src_common_rs_714_16_465a3a::main::{closure#0}>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/common.rs:714:16
   4: kurbo::common::solve_itp::<repro_kurbo_src_common_rs_714_16_465a3a::main::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/common.rs:689:18
   5: repro_kurbo_src_common_rs_714_16_465a3a::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
