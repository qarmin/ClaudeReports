# ruzstd/src/huff0/huff0_decoder.rs:26:26

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: panic
- location: `ruzstd/src/huff0/huff0_decoder.rs:26:26`
- last API call: `ruzstd::huff0::HuffmanDecoder::decode_symbol`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ruzstd = { git = "https://github.com/KillingSpark/zstd-rs", rev = "91a3a6eb4cb8a0252ef73f5a561e30c790635774", features = ["dict_builder", "fuzz_exports", "hash", "rustc-dep-of-std", "std"] }
```

`src/main.rs`:

```rust
use ruzstd::huff0::HuffmanDecoder;
use ruzstd::huff0::HuffmanTable;

fn main() {
    let huffman_table = HuffmanTable::new();
    let mut huffman_decoder = HuffmanDecoder::new(&huffman_table);
    let _ = huffman_decoder.decode_symbol();
}
```

Run with `cargo run` in this directory (package `repro_ruzstd_src_huff0_huff0_decoder_rs_26_26_1630f8`).

## Output

```
thread 'main' (414360) panicked at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_decoder.rs:26:26:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[ruzstd::huff0::huff0_decoder::Entry]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[ruzstd::huff0::huff0_decoder::Entry] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<ruzstd::huff0::huff0_decoder::Entry> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <ruzstd::huff0::huff0_decoder::HuffmanDecoder>::decode_symbol
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_decoder.rs:26:26
   7: repro_ruzstd_src_huff0_huff0_decoder_rs_26_26_1630f8::main
             at ./src/main.rs:7:29
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
