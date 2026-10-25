# zlib-rs/src/lib.rs:242:21

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: panic
- location: `zlib-rs/src/lib.rs:242:21`
- last API call: `zlib_rs::ReturnCode::from<i32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
invalid return code 10
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b" }
```

`src/main.rs`:

```rust
use zlib_rs::ReturnCode;

fn main() {
    let _ = ReturnCode::from(10i32);
}
```

Run with `cargo run` in this directory (package `repro_zlib_rs_src_lib_rs_242_21_04037d`).

## Output

```
thread 'main' (375009) panicked at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/lib.rs:242:21:
invalid return code 10
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <zlib_rs::ReturnCode as core::convert::From<i32>>::from
             at /home/rafal/.cargo/git/checkouts/zlib-rs-0e8d5eb40b54c1a8/7909c0f/zlib-rs/src/lib.rs:242:21
   3: repro_zlib_rs_src_lib_rs_242_21_04037d::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
