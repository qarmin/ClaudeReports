# byteorder-lite/src/lib.rs:1670:9

- crate: https://github.com/image-rs/byteorder-lite @ 7d44da06a48fe10d7e23efd9a932d18a7a2def63
- kind: panic
- location: `byteorder-lite/src/lib.rs:1670:9`
- last API call: `byteorder_lite::BigEndian::default`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
BigEndian default
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
byteorder-lite = { git = "https://github.com/image-rs/byteorder-lite", rev = "7d44da06a48fe10d7e23efd9a932d18a7a2def63" }
```

`src/main.rs`:

```rust
use byteorder_lite::BigEndian;

fn main() {
    let _ = BigEndian::default();
}
```

Run with `cargo run` in this directory (package `repro_byteorder_lite_src_lib_rs_1670_9_137883`).

## Output

```
thread 'main' (328749) panicked at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:1670:9:
BigEndian default
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <byteorder_lite::BigEndian as core::default::Default>::default
             at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:1670:9
   3: repro_byteorder_lite_src_lib_rs_1670_9_137883::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
