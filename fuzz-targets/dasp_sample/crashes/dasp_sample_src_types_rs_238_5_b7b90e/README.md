# dasp_sample/src/types.rs:238:5

- crate: dasp_sample 0.11.0
- kind: panic
- location: `dasp_sample/src/types.rs:238:5`
- source: https://docs.rs/crate/dasp_sample/0.11.0/source/src/types.rs#238
- last API call: `dasp_sample::types::I11::shl<dasp_sample::types::I11>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dasp_sample = { version = "=0.11.0" }
```

`src/main.rs`:

```rust
use dasp_sample::types::I11;
use dasp_sample::types::i11::MAX;

fn main() {
    let _ = <dasp_sample::types::I11 as ::core::ops::Shl<dasp_sample::types::I11>>::shl(MAX, I11::from(69u8));
}
```

Run with `cargo run` in this directory (package `repro_dasp_sample_src_types_rs_238_5_b7b90e`).

## Output

```
thread 'main' (2584977) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:238:5:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <dasp_sample::types::i11::I11 as core::ops::bit::Shl>::shl
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:196:20
   4: repro_dasp_sample_src_types_rs_238_5_b7b90e::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
