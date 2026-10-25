# bigdecimal-rs/src/lib.rs:560:26

- crate: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1
- kind: panic
- location: `bigdecimal-rs/src/lib.rs:560:26`
- last API call: `bigdecimal::Context::round_decimal_ref`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
precision overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bigdecimal = { git = "https://github.com/akubera/bigdecimal-rs", rev = "f0a1f920f222d9776584b26659a3c9326dd1aca1" }
```

`src/main.rs`:

```rust
use bigdecimal::Context;
use bigdecimal::RoundingMode;

fn main() {
    let context = Context::default();
    let context2 = context.with_rounding_mode(RoundingMode::Down);
    let context3 = context2.with_precision(std::num::NonZero::new(u64::MAX).unwrap());
    let big_decimal = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u64(1000u64).unwrap();
    let _ = context3.round_decimal_ref::<bigdecimal::BigDecimalRef>(big_decimal.to_ref());
}
```

Run with `cargo run` in this directory (package `repro_bigdecimal_rs_src_lib_rs_560_26_3e1678`).

## Output

```
thread 'main' (1992145) panicked at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:560:26:
precision overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<i64>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <bigdecimal::BigDecimal>::with_precision_round
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:560:26
   6: <bigdecimal::context::Context>::round_decimal_ref::<bigdecimal::BigDecimalRef>
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/context.rs:99:11
   7: repro_bigdecimal_rs_src_lib_rs_560_26_3e1678::main
             at ./src/main.rs:9:22
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
