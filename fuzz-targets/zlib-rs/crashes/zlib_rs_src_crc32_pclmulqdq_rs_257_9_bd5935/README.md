# zlib-rs/src/crc32/pclmulqdq.rs:257:9

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/crc32/pclmulqdq.rs:257:9`
- last API call: `zlib_rs::crc32::Crc32Fold::fold`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: src.len() >= 64 || !first
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
    crc32_fold.fold(&vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], 1u32);
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_crc32_pclmulqdq_rs_257_9_bd5935`).

## Output

```
thread 'main' (375672) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:257:9:
assertion failed: src.len() >= 64 || !first
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <zlib_rs::crc32::pclmulqdq::Accumulator>::fold_help::<false>
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:257:9
   4: <zlib_rs::crc32::pclmulqdq::Accumulator>::fold
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32/pclmulqdq.rs:43:23
   5: <zlib_rs::crc32::Crc32Fold>::fold
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/crc32.rs:78:47
   6: repro_zlib_rs_src_crc32_pclmulqdq_rs_257_9_bd5935::main
             at ./src/main.rs:5:16
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
