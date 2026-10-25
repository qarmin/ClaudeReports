# zlib-rs/src/deflate.rs:137:9

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/deflate.rs:137:9`
- last API call: `zlib_rs::Deflate::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: StreamError
 right: Ok
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b" }
```

`src/main.rs`:

```rust
use zlib_rs::Deflate;

fn main() {
    let _ = Deflate::new(0i32, false, 0u8);
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_deflate_rs_137_9_ba5d10`).

## Output

```
thread 'main' (373994) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/deflate.rs:137:9:
assertion `left == right` failed
  left: StreamError
 right: Ok
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<zlib_rs::ReturnCode, zlib_rs::ReturnCode>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <zlib_rs::deflate::DeflateStream>::new
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/deflate.rs:137:9
   5: <zlib_rs::stable::Deflate>::new_with_config
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/stable.rs:319:20
   6: <zlib_rs::stable::Deflate>::new
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/stable.rs:308:9
   7: repro_zlib_rs_src_deflate_rs_137_9_ba5d10::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
