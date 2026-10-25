# flate2-rs/src/ffi/c.rs:369:13

- crate: https://github.com/rust-lang/flate2-rs @ 31d6aef2a1478b61d8aeb5931d74f34761add25a
- kind: panic
- location: `flate2-rs/src/ffi/c.rs:369:13`
- last API call: `flate2::write::GzEncoder<Vec<u8>>::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: -2
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
flate2 = { git = "https://github.com/rust-lang/flate2-rs", rev = "31d6aef2a1478b61d8aeb5931d74f34761add25a", features = ["any_c_zlib", "any_impl", "any_zlib", "cloudflare_zlib", "document-features", "libz-ng-sys", "libz-sys", "miniz-sys", "miniz_oxide", "runtime_detection", "rust_backend", "zlib", "zlib-default", "zlib-ng", "zlib-ng-compat", "zlib-rs"] }
```

`src/main.rs`:

```rust
use flate2::Compression;

fn main() {
    let _ = <flate2::write::GzEncoder<Vec<u8>>>::new(vec![], Compression::new(1000u32));
}
```

Run with `cargo run` in this directory (package `repro_flate2_rs_src_ffi_c_rs_369_13_3dd9d7`).

## Output

```
thread 'main' (1414877) panicked at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/ffi/c.rs:369:13:
assertion `left == right` failed
  left: -2
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<i32, i32>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <flate2::ffi::c::Deflate as flate2::ffi::DeflateBackend>::make
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/ffi/c.rs:369:13
   5: <flate2::mem::Compress>::new
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/mem.rs:201:20
   6: flate2::gz::write::gz_encoder::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/gz/write.rs:55:36
   7: <flate2::gz::GzBuilder>::write::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/gz/mod.rs:392:9
   8: <flate2::gz::write::GzEncoder<alloc::vec::Vec<u8>>>::new
             at /home/rafal/.cargo/git/checkouts/flate2-rs-08adeb6c63452319/31d6aef/src/gz/write.rs:71:26
   9: repro_flate2_rs_src_ffi_c_rs_369_13_3dd9d7::main
             at ./src/main.rs:4:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
