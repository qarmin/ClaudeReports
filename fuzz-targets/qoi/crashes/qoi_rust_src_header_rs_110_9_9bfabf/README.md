# qoi-rust/src/header.rs:110:9

- crate: https://github.com/aldanor/qoi-rust @ 81c14c4b637c326a801e40701460b654d46e8fbe
- kind: panic
- location: `qoi-rust/src/header.rs:110:9`
- last API call: `qoi::Header::n_bytes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
qoi = { git = "https://github.com/aldanor/qoi-rust", rev = "81c14c4b637c326a801e40701460b654d46e8fbe" }
```

`src/main.rs`:

```rust
use qoi::Channels;
use qoi::ColorSpace;
use qoi::Header;

fn main() {
    let header = Header { width: u32::MAX, height: u32::MAX, channels: Channels::Rgb, colorspace: ColorSpace::default() };
    let _ = header.n_bytes();
}
```

Run with `cargo run` in this directory (package `repro_qoi_rust_src_header_rs_110_9_9bfabf`).

## Output

```
thread 'main' (1406571) panicked at /home/rafal/.cargo/git/checkouts/qoi-rust-96b45d81ecb0d930/81c14c4/src/header.rs:110:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <qoi::header::Header>::n_bytes
             at /home/rafal/.cargo/git/checkouts/qoi-rust-96b45d81ecb0d930/81c14c4/src/header.rs:110:9
   4: repro_qoi_rust_src_header_rs_110_9_9bfabf::main
             at ./src/main.rs:7:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
