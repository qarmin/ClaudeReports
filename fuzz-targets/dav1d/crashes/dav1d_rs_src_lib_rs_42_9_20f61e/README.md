# dav1d-rs/src/lib.rs:42:9

- crate: https://github.com/rust-av/dav1d-rs @ 03477a36c3de4f2aacbab81a8951150ffdb9c4c3
- kind: panic
- location: `dav1d-rs/src/lib.rs:42:9`
- last API call: `dav1d::Error::from<i32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: err < 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
dav1d = { git = "https://github.com/rust-av/dav1d-rs", rev = "03477a36c3de4f2aacbab81a8951150ffdb9c4c3" }
```

`src/main.rs`:

```rust
use dav1d::Error;

fn main() {
    let _ = Error::from(0i32);
}
```

Run with `cargo run` in this directory (package `repro_dav1d_rs_src_lib_rs_42_9_20f61e`).

## Output

```
thread 'main' (1417152) panicked at /home/rafal/.cargo/git/checkouts/dav1d-rs-3d67a063d7689d7c/03477a3/src/lib.rs:42:9:
assertion failed: err < 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <dav1d::Error as core::convert::From<i32>>::from
             at /home/rafal/.cargo/git/checkouts/dav1d-rs-3d67a063d7689d7c/03477a3/src/lib.rs:42:9
   4: repro_dav1d_rs_src_lib_rs_42_9_20f61e::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
