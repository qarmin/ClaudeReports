# num-bigint/src/prime.rs:187:34

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/prime.rs:187:34`
- last API call: `num_bigint_dig::prime::probably_prime_miller_rabin`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
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
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&BigUint::from(1usize), 0usize, false);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_prime_rs_187_34_836f2e`).

## Output

```
thread 'main' (1267831) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/prime.rs:187:34:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<usize>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: num_bigint_dig::prime::probably_prime_miller_rabin
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/prime.rs:187:34
   6: repro_num_bigint_src_prime_rs_187_34_836f2e::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
