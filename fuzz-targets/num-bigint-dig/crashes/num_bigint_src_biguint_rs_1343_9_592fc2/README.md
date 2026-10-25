# num-bigint/src/biguint.rs:1343:9

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:1343:9`
- last API call: `num_bigint_dig::BigUint::neg`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
explicit panic
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <num_bigint_dig::BigUint as ::core::ops::Neg>::neg(num_bigint_dig::algorithms::mul3(&[], &[]));
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_1343_9_592fc2`).

## Output

```
thread 'main' (1266891) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:1343:9:
explicit panic
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <num_bigint_dig::biguint::BigUint as core::ops::arith::Neg>::neg
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:1343:9
   4: repro_num_bigint_src_biguint_rs_1343_9_592fc2::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
