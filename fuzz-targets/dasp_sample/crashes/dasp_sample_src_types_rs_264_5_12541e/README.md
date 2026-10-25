# dasp_sample/src/types.rs:264:5

- crate: dasp_sample 0.11.0
- kind: panic
- location: `dasp_sample/src/types.rs:264:5`
- source: https://docs.rs/crate/dasp_sample/0.11.0/source/src/types.rs#264
- last API call: `dasp_sample::types::U11::div<dasp_sample::types::U11>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to divide by zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dasp_sample = { version = "=0.11.0" }
```

`src/main.rs`:

```rust
use dasp_sample::types::u11::MIN;

fn main() {
    let _ = <dasp_sample::types::U11 as ::core::ops::Div<dasp_sample::types::U11>>::div(MIN, MIN);
}
```

Run with `cargo run` in this directory (package `repro_dasp_sample_src_types_rs_264_5_12541e`).

## Output

```
thread 'main' (2585325) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:264:5:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <dasp_sample::types::u11::U11 as core::ops::arith::Div>::div
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:171:20
   4: repro_dasp_sample_src_types_rs_264_5_12541e::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
