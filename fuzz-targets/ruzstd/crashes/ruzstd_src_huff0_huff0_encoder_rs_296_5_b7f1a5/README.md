# ruzstd/src/huff0/huff0_encoder.rs:296:5

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: panic
- location: `ruzstd/src/huff0/huff0_encoder.rs:296:5`
- last API call: `ruzstd::huff0::huff0_encoder::HuffmanTable::build_from_counts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: amount >= 2
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
    let _ = HuffmanTable::build_from_counts(&[]);
}
```

Run with `cargo run` in this directory (package `repro_ruzstd_src_huff0_huff0_encoder_rs_296_5_b7f1a5`).

## Output

```
thread 'main' (414262) panicked at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:296:5:
assertion failed: amount >= 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: ruzstd::huff0::huff0_encoder::distribute_weights
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:296:5
   4: <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_counts
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:176:27
   5: repro_ruzstd_src_huff0_huff0_encoder_rs_296_5_b7f1a5::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
