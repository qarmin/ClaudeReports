# num-bigint/src/biguint.rs:2024:5

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:2024:5`
- last API call: `num_bigint_dig::BigInt::to_str_radix`
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
use num_bigint_dig::BigInt;

fn main() {
    let big_int = <num_bigint_dig::BigInt as ::core::ops::Rem<u8>>::rem(BigInt::from(0u32), 0u8);
    let _ = big_int.to_str_radix(0u32);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_2024_5_556326`).

## Output

```
thread 'main' (1266517) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2024:5:
The radix must be within 2...36
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: num_bigint_dig::biguint::to_str_radix_reversed
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2024:5
   3: <num_bigint_dig::bigint::BigInt>::to_str_radix
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/bigint.rs:3006:21
   4: repro_num_bigint_src_biguint_rs_2024_5_556326::main
             at ./src/main.rs:5:21
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
