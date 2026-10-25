# num-bigint/src/biguint.rs:2214:9

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:2214:9`
- last API call: `num_bigint_dig::BigInt::from_radix_be`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
The radix must be within 2...256
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2" }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigInt;
use num_bigint_dig::Sign;

fn main() {
    let _ = BigInt::from_radix_be(Sign::Minus, &vec![], 0u32);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_2214_9_451c63`).

## Output

```
thread 'main' (1267146) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2214:9:
The radix must be within 2...256
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <num_bigint_dig::biguint::BigUint>::from_radix_be
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2214:9
   3: <num_bigint_dig::bigint::BigInt>::from_radix_be
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/bigint.rs:2884:9
   4: repro_num_bigint_src_biguint_rs_2214_9_451c63::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
