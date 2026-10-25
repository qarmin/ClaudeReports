# num-bigint/src/algorithms/jacobi.rs:10:9

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/algorithms/jacobi.rs:10:9`
- last API call: `num_bigint_dig::algorithms::jacobi`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
invalid arguments, y must be an odd integer,but got BigInt { sign: NoSign, data: BigUint { data: [] } }
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
use num_bigint_dig::BigUint;
use num_bigint_dig::Sign;

fn main() {
    let big_int = BigInt::from_biguint(Sign::Minus, BigUint::from(0u8));
    let _ = num_bigint_dig::algorithms::jacobi(&big_int, &BigInt::from_signed_bytes_be(&vec![]));
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_algorithms_jacobi_rs_10_9_374abd`).

## Output

```
thread 'main' (1266995) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/jacobi.rs:10:9:
invalid arguments, y must be an odd integer,but got BigInt { sign: NoSign, data: BigUint { data: [] } }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: num_bigint_dig::algorithms::jacobi::jacobi
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/jacobi.rs:10:9
   3: repro_num_bigint_src_algorithms_jacobi_rs_10_9_374abd::main
             at ./src/main.rs:7:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
