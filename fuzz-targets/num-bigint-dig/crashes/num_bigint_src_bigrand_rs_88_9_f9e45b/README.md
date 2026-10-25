# num-bigint/src/bigrand.rs:88:9

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/bigrand.rs:88:9`
- last API call: `num_bigint_dig::prime::probably_prime_miller_rabin`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !bound.is_zero()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2", features = ["arbitrary", "fuzz", "i128", "nightly", "prime", "rand", "serde", "std", "u64_digit", "zeroize"] }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&<num_bigint_dig::BigUint as ::core::ops::Div<u64>>::div(BigUint::new(vec![45u32]), 21u64), 1usize, false);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_bigrand_rs_88_9_f9e45b`).

## Output

```
thread 'main' (1267743) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/bigrand.rs:88:9:
assertion failed: !bound.is_zero()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <rand::rngs::std::StdRng as num_bigint_dig::bigrand::RandBigInt>::gen_biguint_below
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/bigrand.rs:88:9
   4: num_bigint_dig::prime::probably_prime_miller_rabin
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/prime.rs:198:17
   5: repro_num_bigint_src_bigrand_rs_88_9_f9e45b::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
