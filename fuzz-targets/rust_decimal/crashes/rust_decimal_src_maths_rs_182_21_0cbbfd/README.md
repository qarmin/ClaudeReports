# rust-decimal/src/maths.rs:182:21

- crate: https://github.com/paupino/rust-decimal @ b23fb077e301e77cb0ac36cc1ac4df1fba6c45d5
- kind: panic
- location: `rust-decimal/src/maths.rs:182:21`
- last API call: `rust_decimal::Decimal::exp_with_tolerance`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Exp overflowed
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rust_decimal = { git = "https://github.com/paupino/rust-decimal", rev = "b23fb077e301e77cb0ac36cc1ac4df1fba6c45d5", features = ["align16", "borsh", "bytemuck", "c-repr", "db-diesel-mysql", "db-diesel-postgres", "db-diesel2-mysql", "db-diesel2-postgres", "db-postgres", "db-tokio-postgres", "diesel", "macros", "maths", "maths-nopanic", "proptest", "rand-0_9", "rust-fuzz", "serde", "serde-arbitrary-precision", "serde-bincode", "serde-float", "serde-str", "serde-with-arbitrary-precision", "serde-with-float", "serde-with-str", "serde_json", "std", "tokio-pg", "wasm", "zmij"] }
```

`src/main.rs`:

```rust
use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::exp_with_tolerance(&Decimal::from(100u8), Decimal::PI);
}
```

Run with `cargo run` in this directory (package `repro_rust_decimal_src_maths_rs_182_21_0cbbfd`).

## Output

```
thread 'main' (1998627) panicked at /home/rafal/.cargo/git/checkouts/rust-decimal-0d7f101410b99e07/b23fb07/src/maths.rs:182:21:
Exp overflowed
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps>::exp_with_tolerance
             at /home/rafal/.cargo/git/checkouts/rust-decimal-0d7f101410b99e07/b23fb07/src/maths.rs:182:21
   3: repro_rust_decimal_src_maths_rs_182_21_0cbbfd::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
