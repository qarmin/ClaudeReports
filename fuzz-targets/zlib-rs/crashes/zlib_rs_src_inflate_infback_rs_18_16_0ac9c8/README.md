# zlib-rs/src/inflate/infback.rs:18:16

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/inflate/infback.rs:18:16`
- last API call: `zlib_rs::inflate::back_init`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b", features = ["ZLIB_DEBUG", "__internal-api", "__internal-fuzz", "__internal-fuzz-disable-checksum", "__internal-test", "arbitrary", "avx512", "c-allocator", "lsx", "quickcheck", "rust-allocator", "std", "vpclmulqdq"] }
```

`src/main.rs`:

```rust
use zlib_rs::InflateConfig;
use zlib_rs::inflate::Window;

fn main() {
    let mut z_stream = zlib_rs::c_api::z_stream::default();
    let _ = zlib_rs::inflate::back_init(&mut z_stream, InflateConfig { window_bits: 1000i32 }, Window::empty());
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_inflate_infback_rs_18_16_0ac9c8`).

## Output

```
thread 'main' (374565) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/inflate/infback.rs:18:16:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: zlib_rs::inflate::infback::back_init
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/inflate/infback.rs:18:16
   4: repro_zlib_rs_src_inflate_infback_rs_18_16_0ac9c8::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
