# bigdecimal-rs/src/arithmetic/multiplication.rs:686:19

- crate: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1
- kind: panic
- location: `bigdecimal-rs/src/arithmetic/multiplication.rs:686:19`
- last API call: `bigdecimal::BigDecimal::powi`
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

fn main() {
    let _ = BigDecimal::from(2i64).powi(i64::MIN);
}
```

Run with `cargo run` in this directory (package `repro_bigdecimal_rs_src_arithmetic_multiplication_rs_686_19_a02f63`).

## Output

```
thread 'main' (1992078) panicked at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/multiplication.rs:686:19:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bigdecimal::arithmetic::multiplication::mul_scaled_slices_truncating_into::<bigdecimal::bigdigit::radix::RADIX_10p19_u64, bigdecimal::bigdigit::endian::LittleEndian, bigdecimal::bigdigit::endian::LittleEndian, bigdecimal::bigdigit::endian::LittleEndian>
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/multiplication.rs:686:19
   4: bigdecimal::arithmetic::pow::pow_u64_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:158:33
   5: bigdecimal::arithmetic::pow::powi_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:54:13
   6: bigdecimal::arithmetic::pow::impl_powi_with_context::<bigdecimal::BigDecimalRef>
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/arithmetic/pow.rs:29:5
   7: <bigdecimal::BigDecimal>::powi_with_context
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:798:9
   8: <bigdecimal::BigDecimal>::powi
             at /home/rafal/.cargo/git/checkouts/bigdecimal-rs-80ee76ef09ec0008/f0a1f92/src/lib.rs:787:14
   9: repro_bigdecimal_rs_src_arithmetic_multiplication_rs_686_19_a02f63::main
             at ./src/main.rs:4:36
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
