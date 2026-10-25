# out of memory in num_bigint_dig::algorithms::div_rem_digit

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: out of memory
- last API call: `num_bigint_dig::algorithms::div_rem_digit`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling num_bigint_dig::algorithms::div_rem_digit
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
    let _ = num_bigint_dig::algorithms::div_rem_digit(<num_bigint_dig::BigUint as ::core::ops::Shl<usize>>::shl(BigUint::from_bytes_be(&vec![]), usize::MAX), 1u64);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_num_bigint_dig_algorithms_div_rem_digit_a4b4d8`).

## Output

```
memory allocation of 2305843009213693952 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: smallvec::infallible::<()>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.3/src/lib.rs:331:57
   5: <smallvec::SmallVec<[u64; 4]>>::reserve_exact
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.3/src/lib.rs:1375:9
   6: <smallvec::SmallVec<[u64; 4]>>::with_capacity
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.3/src/lib.rs:880:11
   7: num_bigint_dig::algorithms::shl::biguint_shl
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/shl.rs:16:28
   8: <num_bigint_dig::biguint::BigUint as core::ops::bit::Shl<usize>>::shl
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/biguint.rs:430:9
   9: repro_out_of_memory_in_num_bigint_dig_algorithms_div_rem_digit_a4b4d8::main
             at ./src/main.rs:4:55
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
