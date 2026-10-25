# rust-brotli/src/enc/entropy_encode.rs:194:25

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/entropy_encode.rs:194:25`
- last API call: `brotli::enc::brotli_bit_stream::BrotliStoreHuffmanTree`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 2 but the index is 2
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::brotli_bit_stream::BrotliStoreHuffmanTree;
use brotli::enc::entropy_encode::HuffmanTree;

fn main() {
    BrotliStoreHuffmanTree(&vec![], 0usize, &mut vec![HuffmanTree::default(), HuffmanTree::new(0u32, 0i16, 0i16)], &mut 0usize, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_entropy_encode_rs_194_25_1abe5b`).

## Output

```
thread 'main' (1254802) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:194:25:
index out of bounds: the len is 2 but the index is 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::entropy_encode::BrotliCreateHuffmanTree
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:194:25
   4: brotli::enc::brotli_bit_stream::BrotliStoreHuffmanTree
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/brotli_bit_stream.rs:881:5
   5: repro_rust_brotli_src_enc_entropy_encode_rs_194_25_1abe5b::main
             at ./src/main.rs:5:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
