# bigdecimal-rs/src/arithmetic/inverse.rs:73:77

- crate: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1
- kind: panic
- location: `bigdecimal-rs/src/arithmetic/inverse.rs:73:77`
- last API call: `bigdecimal::Context::invert`
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
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Mul<i16>>::mul(BigDecimal::try_from(1.0f32).unwrap(), 1i16);
    let big_decimal2 = big_decimal.sqrt().unwrap();
    let big_decimal_ref = big_decimal2.to_ref();
    let _ = Context::new(std::num::NonZero::new(u64::MAX).unwrap(), RoundingMode::HalfDown).invert::<bigdecimal::BigDecimalRef>(big_decimal_ref);
}
```

Run with `cargo run` in this directory (package `repro_bigdecimal_rs_src_arithmetic_inverse_rs_73_77_1ffea9`).

## Output

```
thread 'main' (1992221) panicked at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/inverse.rs:73:77:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bigdecimal::arithmetic::inverse::impl_inverse_uint_scale
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/inverse.rs:73:77
   4: bigdecimal::arithmetic::inverse::inverse_scaled_uint_with_context::<bigdecimal::BigDecimalRef>
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/inverse.rs:12:5
   5: <bigdecimal::BigDecimalRef>::inverse_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:1400:9
   6: <bigdecimal::context::Context>::invert::<bigdecimal::BigDecimalRef>
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/context.rs:184:18
   7: repro_bigdecimal_rs_src_arithmetic_inverse_rs_73_77_1ffea9::main
             at ./src/main.rs:9:93
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
