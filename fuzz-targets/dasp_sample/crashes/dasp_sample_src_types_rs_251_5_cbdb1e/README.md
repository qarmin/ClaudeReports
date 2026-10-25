# dasp_sample/src/types.rs:251:5

- crate: dasp_sample 0.11.0
- kind: panic
- location: `dasp_sample/src/types.rs:251:5`
- source: https://docs.rs/crate/dasp_sample/0.11.0/source/src/types.rs#251
- last API call: `dasp_sample::I24::rem<dasp_sample::I24>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to calculate the remainder with a divisor of zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dasp_sample = { version = "=0.11.0" }
```

`src/main.rs`:

```rust
use dasp_sample::I24;

fn main() {
    let _ = <dasp_sample::I24 as ::core::ops::Rem<dasp_sample::I24>>::rem(dasp_sample::conv::u24::to_i24(dasp_sample::conv::i64::to_u24(0i64)), I24::from(0u16));
}
```

Run with `cargo run` in this directory (package `repro_dasp_sample_src_types_rs_251_5_cbdb1e`).

## Output

```
thread 'main' (2584855) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:251:5:
attempt to calculate the remainder with a divisor of zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_rem_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <dasp_sample::types::i24::I24 as core::ops::arith::Rem>::rem
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dasp_sample-0.11.0/src/types.rs:187:20
   4: repro_dasp_sample_src_types_rs_251_5_cbdb1e::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
