# dasp_sample/src/types.rs:282:5

- crate: dasp_sample 0.11.0
- kind: panic
- location: `dasp_sample/src/types.rs:282:5`
- source: https://docs.rs/crate/dasp_sample/0.11.0/source/src/types.rs#282
- last API call: `dasp_sample::U48::add<dasp_sample::U48>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
arithmetic operation overflowed
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dasp_sample = { version = "=0.11.0" }
```

`src/main.rs`:

```rust
use dasp_sample::types::i24::MAX;

fn main() {
    let _ = <dasp_sample::U48 as ::core::ops::Add<dasp_sample::U48>>::add(dasp_sample::conv::i24::to_u48(MAX), dasp_sample::conv::u64::to_u48(1000000000000000000u64));
}
```

Run with `cargo run` in this directory (package `repro_dasp_sample_src_types_rs_282_5_dcc374`).

## Output

```
thread 'main' (2585055) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:282:5:
arithmetic operation overflowed
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<dasp_sample::types::u48::U48>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <dasp_sample::types::u48::U48 as core::ops::arith::Add>::add
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:136:47
   6: repro_dasp_sample_src_types_rs_282_5_dcc374::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
