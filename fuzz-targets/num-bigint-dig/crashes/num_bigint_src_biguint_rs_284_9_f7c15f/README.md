# num-bigint/src/biguint.rs:284:9

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:284:9`
- last API call: `num_bigint_dig::BigUint::parse_bytes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
The radix must be within 2...36
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2" }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigUint;

fn main() {
    let _ = BigUint::parse_bytes(&vec![], 0u32);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_284_9_f7c15f`).

## Output

```
thread 'main' (1266746) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:284:9:
The radix must be within 2...36
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <num_bigint_dig::biguint::BigUint as num_traits::Num>::from_str_radix
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:284:9
   3: <num_bigint_dig::biguint::BigUint>::parse_bytes::{closure#0}
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2194:27
   4: <core::option::Option<&str>>::and_then::<num_bigint_dig::biguint::BigUint, <num_bigint_dig::biguint::BigUint>::parse_bytes::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1542:24
   5: <num_bigint_dig::biguint::BigUint>::parse_bytes
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2194:14
   6: repro_num_bigint_src_biguint_rs_284_9_f7c15f::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
