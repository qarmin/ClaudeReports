# zlib-rs/src/crc32/pclmulqdq.rs:260:13

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/crc32/pclmulqdq.rs:260:13`
- last API call: `zlib_rs::crc32::Crc32Fold::fold_copy`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: dst and src must be the same length
  left: 0
 right: 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b", features = ["ZLIB_DEBUG", "__internal-api", "__internal-fuzz", "__internal-fuzz-disable-checksum", "__internal-test", "arbitrary", "avx512", "c-allocator", "lsx", "quickcheck", "rust-allocator", "std", "vpclmulqdq"] }
```

`src/main.rs`:

```rust
use zlib_rs::crc32::Crc32Fold;

fn main() {
    let mut crc32_fold = Crc32Fold::new();
    crc32_fold.fold_copy(&mut vec![], &vec![32u8]);
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_crc32_pclmulqdq_rs_260_13_a5a021`).

## Output

```
thread 'main' (375082) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:260:13:
assertion `left == right` failed: dst and src must be the same length
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <zlib_rs::crc32::pclmulqdq::Accumulator>::fold_help::<true>
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:260:13
   5: <zlib_rs::crc32::pclmulqdq::Accumulator>::fold_copy
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:47:23
   6: <zlib_rs::crc32::Crc32Fold>::fold_copy
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32.rs:105:39
   7: repro_zlib_rs_src_crc32_pclmulqdq_rs_260_13_a5a021::main
             at ./src/main.rs:5:16
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
