# ruzstd/src/huff0/huff0_encoder.rs:230:13

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: panic
- location: `ruzstd/src/huff0/huff0_encoder.rs:230:13`
- last API call: `ruzstd::huff0::huff0_encoder::HuffmanTable::build_from_weights`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
This is an internal error
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ruzstd = { git = "https://github.com/KillingSpark/zstd-rs", rev = "91a3a6eb4cb8a0252ef73f5a561e30c790635774", features = ["dict_builder", "fuzz_exports", "hash", "rustc-dep-of-std", "std"] }
```

`src/main.rs`:

```rust
use ruzstd::huff0::huff0_encoder::HuffmanTable;

fn main() {
    let _ = HuffmanTable::build_from_weights(&[]);
}
```

Run with `cargo run` in this directory (package `repro_ruzstd_src_huff0_huff0_encoder_rs_230_13_642d3c`).

## Output

```
thread 'main' (414181) panicked at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:230:13:
This is an internal error
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:230:13
   4: repro_ruzstd_src_huff0_huff0_encoder_rs_230_13_642d3c::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
