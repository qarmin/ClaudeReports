# hamming-bitwise-fast/src/slice.rs:42:5

- crate: https://github.com/emschwartz/hamming-bitwise-fast @ 1ac2bc123d91ddad9dccb7e8a13f0f107046cd87
- kind: panic
- location: `hamming-bitwise-fast/src/slice.rs:42:5`
- last API call: `hamming_bitwise_fast::hamming_bitwise_fast`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 0
 right: 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hamming-bitwise-fast = { git = "https://github.com/emschwartz/hamming-bitwise-fast", rev = "1ac2bc123d91ddad9dccb7e8a13f0f107046cd87" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = hamming_bitwise_fast::hamming_bitwise_fast(&vec![], &vec![0u8]);
}
```

Run with `cargo run` in this directory (package `repro_hamming_bitwise_fast_src_slice_rs_42_5_86aa0f`).

## Output

```
thread 'main' (587801) panicked at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/slice.rs:42:5:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: hamming_bitwise_fast::slice::distance::distance_avx_avx2_avx512f_avx512vl_avx512vpopcntdq_f16c_fma_popcnt_sse_sse2_sse3_sse41_ssse3_version::__safe_inner
             at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/slice.rs:42:5
   5: hamming_bitwise_fast::slice::distance::distance_avx_avx2_avx512f_avx512vl_avx512vpopcntdq_f16c_fma_popcnt_sse_sse2_sse3_sse41_ssse3_version
             at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/slice.rs:31:5
   6: hamming_bitwise_fast::slice::distance::__resolver_fn
             at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/slice.rs:31:5
   7: hamming_bitwise_fast::slice::distance
             at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/slice.rs:31:5
   8: hamming_bitwise_fast::hamming_bitwise_fast
             at /home/rafal/.cargo/git/checkouts/hamming-bitwise-fast-e6608c0df0dca152/1ac2bc1/src/lib.rs:123:5
   9: repro_hamming_bitwise_fast_src_slice_rs_42_5_86aa0f::main
             at ./src/main.rs:2:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
