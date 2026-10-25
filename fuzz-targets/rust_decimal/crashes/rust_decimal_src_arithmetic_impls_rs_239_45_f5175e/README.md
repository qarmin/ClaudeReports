# rust-decimal/src/arithmetic_impls.rs:239:45

- crate: https://github.com/paupino/rust-decimal @ b23fb077e301e77cb0ac36cc1ac4df1fba6c45d5
- kind: panic
- location: `rust-decimal/src/arithmetic_impls.rs:239:45`
- last API call: `rust_decimal::Decimal::div<rust_decimal::Decimal>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Division by zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rust_decimal = { git = "https://github.com/paupino/rust-decimal", rev = "b23fb077e301e77cb0ac36cc1ac4df1fba6c45d5" }
```

`src/main.rs`:

```rust
use rust_decimal::Decimal;

fn main() {
    let u32_v = Decimal::MAX_SCALE;
    let _ = <rust_decimal::Decimal as ::core::ops::Div<rust_decimal::Decimal>>::div(Decimal::from(u32_v), Decimal::try_from(0.0f64).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_rust_decimal_src_arithmetic_impls_rs_239_45_f5175e`).

## Output

```
thread 'main' (1998884) panicked at src/main.rs:5:13:
Division by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <&rust_decimal::decimal::Decimal as core::ops::arith::Div>::div
             at /home/rafal/.cargo/git/checkouts/rust-decimal-0d7f101410b99e07/b23fb07/src/arithmetic_impls.rs:239:45
   3: <rust_decimal::decimal::Decimal as core::ops::arith::Div>::div
             at /home/rafal/.cargo/git/checkouts/rust-decimal-0d7f101410b99e07/b23fb07/src/arithmetic_impls.rs:164:25
   4: repro_rust_decimal_src_arithmetic_impls_rs_239_45_f5175e::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
