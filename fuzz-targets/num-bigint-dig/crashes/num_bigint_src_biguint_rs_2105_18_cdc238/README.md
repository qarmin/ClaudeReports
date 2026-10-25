# num-bigint/src/biguint.rs:2105:18

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/biguint.rs:2105:18`
- last API call: `num_bigint_dig::BigInt::get_limb`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
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
    let big_int = <num_bigint_dig::BigInt as ::core::ops::Div<i64>>::div(BigInt::default(), 1i64);
    let _ = big_int.get_limb(0usize);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_rs_2105_18_cdc238`).

## Output

```
thread 'main' (1267261) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2105:18:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[u64]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[u64] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <smallvec::SmallVec<[u64; 4]> as core::ops::index::Index<usize>>::index
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.3/src/lib.rs:2288:18
   6: <num_bigint_dig::biguint::BigUint>::get_limb
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:2105:18
   7: <num_bigint_dig::bigint::BigInt>::get_limb
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/bigint.rs:3160:19
   8: repro_num_bigint_src_biguint_rs_2105_18_cdc238::main
             at ./src/main.rs:5:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
