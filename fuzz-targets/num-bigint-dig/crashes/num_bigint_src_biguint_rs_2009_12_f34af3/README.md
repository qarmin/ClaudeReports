# num-bigint/src/biguint.rs:2009:12

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:2009:12`
- last API call: `num_bigint_dig::BigUint::to_radix_le`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to calculate the remainder with a divisor of zero
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
    let big_uint = BigUint::new(vec![1u32]);
    let _ = big_uint.to_radix_le(1u32);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_2009_12_f34af3`).

## Output

```
thread 'main' (1267097) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2009:12:
attempt to calculate the remainder with a divisor of zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_rem_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: num_bigint_dig::biguint::to_radix_le
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2009:12
   4: <num_bigint_dig::biguint::BigUint>::to_radix_le
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2374:9
   5: repro_num_bigint_src_biguint_rs_2009_12_f34af3::main
             at ./src/main.rs:5:22
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
