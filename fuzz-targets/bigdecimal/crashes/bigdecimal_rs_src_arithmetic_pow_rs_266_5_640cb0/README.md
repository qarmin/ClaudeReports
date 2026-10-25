# bigdecimal-rs/src/arithmetic/pow.rs:266:5

- crate: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1
- kind: panic
- location: `bigdecimal-rs/src/arithmetic/pow.rs:266:5`
- last API call: `bigdecimal::BigDecimalRef::exp`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bigdecimal = { git = "https://github.com/akubera/bigdecimal-rs", rev = "f0a1f920f222d9776584b26659a3c9326dd1aca1" }
```

`src/main.rs`:

```rust
fn main() {
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Mul<i64>>::mul(<bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u64(10000000000000000000u64).unwrap(), 65535i64);
    let big_decimal_ref = big_decimal.to_ref();
    let _ = big_decimal_ref.exp();
}
```

Run with `cargo run` in this directory (package `repro_bigdecimal_rs_src_arithmetic_pow_rs_266_5_640cb0`).

## Output

```
thread 'main' (1991527) panicked at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:266:5:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:266:5
   4: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
   5: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
   6: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
   7: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
   8: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
   9: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  10: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  11: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  12: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  13: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  14: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  15: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  16: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  17: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  18: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  19: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  20: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  21: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  22: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  23: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  24: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  25: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
... (75 lines omitted)
  63: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  64: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  65: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  66: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  67: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  68: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  69: bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:272:5
  70: bigdecimal::arithmetic::exp::impl_exp
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/exp.rs:67:5
  71: <bigdecimal::BigDecimalRef>::exp_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:1444:9
  72: <bigdecimal::BigDecimalRef>::exp
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:1438:14
  73: repro_bigdecimal_rs_src_arithmetic_pow_rs_266_5_640cb0::main
             at ./src/main.rs:4:29
  74: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
