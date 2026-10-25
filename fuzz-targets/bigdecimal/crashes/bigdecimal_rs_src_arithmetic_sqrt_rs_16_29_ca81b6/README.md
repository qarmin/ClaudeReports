# bigdecimal-rs/src/arithmetic/sqrt.rs:16:29

- crate: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1
- kind: panic
- location: `bigdecimal-rs/src/arithmetic/sqrt.rs:16:29`
- last API call: `bigdecimal::BigDecimalRef::sqrt_copysign_with_context`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bigdecimal = { git = "https://github.com/akubera/bigdecimal-rs", rev = "f0a1f920f222d9776584b26659a3c9326dd1aca1" }
```

`src/main.rs`:

```rust
use bigdecimal::BigDecimal;
use bigdecimal::Context;
use bigdecimal::RoundingMode;

fn main() {
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Neg>::neg(BigDecimal::from(0i32));
    let big_decimal2 = big_decimal.with_scale_round(0i64, RoundingMode::HalfEven);
    let big_decimal3 = big_decimal2.with_scale(0i64);
    let big_decimal_ref = big_decimal3.to_ref();
    let _ = big_decimal_ref.sqrt_copysign_with_context(&Context::default().with_precision(std::num::NonZero::new(u64::MAX).unwrap()));
}
```

Run with `cargo run` in this directory (package `repro_bigdecimal_rs_src_arithmetic_sqrt_rs_16_29_ca81b6`).

## Output

```
thread 'main' (1991574) panicked at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/sqrt.rs:16:29:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bigdecimal::arithmetic::sqrt::impl_sqrt
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/sqrt.rs:16:29
   4: <bigdecimal::BigDecimalRef>::sqrt_copysign_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:1426:26
   5: repro_bigdecimal_rs_src_arithmetic_sqrt_rs_16_29_ca81b6::main
             at ./src/main.rs:10:29
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
