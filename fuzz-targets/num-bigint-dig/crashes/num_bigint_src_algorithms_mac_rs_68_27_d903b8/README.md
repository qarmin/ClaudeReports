# num-bigint/src/algorithms/mac.rs:68:27

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/algorithms/mac.rs:68:27`
- last API call: `num_bigint_dig::algorithms::mac3`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 1 out of range for slice of length 0
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
    num_bigint_dig::algorithms::mac3(&mut vec![], &[0u64, 0u64], &[0u64, 0u64]);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_algorithms_mac_rs_68_27_d903b8`).

## Output

```
thread 'main' (1267476) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/mac.rs:68:27:
range start index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u64]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:558:13
   4: <[u64] as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: num_bigint_dig::algorithms::mac::long
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/mac.rs:68:27
   6: num_bigint_dig::algorithms::mac::mac3
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/mac.rs:57:9
   7: repro_num_bigint_src_algorithms_mac_rs_68_27_d903b8::main
             at ./src/main.rs:2:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
