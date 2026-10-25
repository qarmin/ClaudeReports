# zlib-rs/src/inflate/window.rs:41:9

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/inflate/window.rs:41:9`
- last API call: `zlib_rs::inflate::Window::buffer_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.buf.len().is_power_of_two()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b", features = ["ZLIB_DEBUG", "__internal-api", "__internal-fuzz", "__internal-fuzz-disable-checksum", "__internal-test", "arbitrary", "avx512", "c-allocator", "lsx", "quickcheck", "rust-allocator", "std", "vpclmulqdq"] }
```

`src/main.rs`:

```rust
use zlib_rs::inflate::Window;

fn main() {
    let window = Window::empty();
    let _ = window.buffer_size();
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_inflate_window_rs_41_9_d4a7c1`).

## Output

```
thread 'main' (375198) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/inflate/window.rs:41:9:
assertion failed: self.buf.len().is_power_of_two()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <zlib_rs::inflate::window::Window>::buffer_size
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/inflate/window.rs:41:9
   4: repro_zlib_rs_src_inflate_window_rs_41_9_d4a7c1::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
