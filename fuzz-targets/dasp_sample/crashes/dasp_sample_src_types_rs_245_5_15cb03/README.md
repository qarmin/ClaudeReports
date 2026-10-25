# dasp_sample/src/types.rs:245:5

- crate: dasp_sample 0.11.0
- kind: panic
- location: `dasp_sample/src/types.rs:245:5`
- source: https://docs.rs/crate/dasp_sample/0.11.0/source/src/types.rs#245
- last API call: `dasp_sample::types::I20::shr<dasp_sample::types::I20>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift right with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dasp_sample = { version = "=0.11.0" }
```

`src/main.rs`:

```rust
use dasp_sample::types::I20;
use dasp_sample::types::i20::MIN;

fn main() {
    let i20 = I20::from(0u16);
    let _ = <dasp_sample::types::I20 as ::core::ops::Shr<dasp_sample::types::I20>>::shr(i20, MIN);
}
```

Run with `cargo run` in this directory (package `repro_dasp_sample_src_types_rs_245_5_15cb03`).

## Output

```
thread 'main' (2584610) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:245:5:
attempt to shift right with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shr_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <dasp_sample::types::i20::I20 as core::ops::bit::Shr>::shr
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:205:20
   4: repro_dasp_sample_src_types_rs_245_5_15cb03::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
