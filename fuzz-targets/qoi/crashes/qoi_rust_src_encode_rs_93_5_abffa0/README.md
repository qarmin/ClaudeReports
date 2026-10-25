# qoi-rust/src/encode.rs:93:5

- crate: https://github.com/aldanor/qoi-rust @ 81c14c4b637c326a801e40701460b654d46e8fbe
- kind: panic
- location: `qoi-rust/src/encode.rs:93:5`
- last API call: `qoi::encode_max_len`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
qoi = { git = "https://github.com/aldanor/qoi-rust", rev = "81c14c4b637c326a801e40701460b654d46e8fbe" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = qoi::encode_max_len(1000000000u32, 1000000000u32, 32u8);
}
```

Run with `cargo run` in this directory (package `repro_qoi_rust_src_encode_rs_93_5_abffa0`).

## Output

```
thread 'main' (1406617) panicked at /home/rafal/.cargo/git/checkouts/qoi-rust-96b45d81ecb0d930/81c14c4/src/encode.rs:93:5:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: qoi::encode::encode_max_len::<u8>
             at /home/rafal/.cargo/git/checkouts/qoi-rust-96b45d81ecb0d930/81c14c4/src/encode.rs:93:5
   4: repro_qoi_rust_src_encode_rs_93_5_abffa0::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
