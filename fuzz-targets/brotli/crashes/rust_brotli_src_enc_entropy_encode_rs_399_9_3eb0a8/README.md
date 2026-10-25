# rust-brotli/src/enc/entropy_encode.rs:399:9

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/entropy_encode.rs:399:9`
- last API call: `brotli::enc::entropy_encode::BrotliWriteHuffmanTree`
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
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::entropy_encode::BrotliWriteHuffmanTree;

fn main() {
    BrotliWriteHuffmanTree(&vec![35u8], 1usize, &mut 0usize, &mut vec![], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_entropy_encode_rs_399_9_3eb0a8`).

## Output

```
thread 'main' (1254967) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:399:9:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::entropy_encode::BrotliWriteHuffmanTreeRepetitions
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:399:9
   4: brotli::enc::entropy_encode::BrotliWriteHuffmanTree
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:513:13
   5: repro_rust_brotli_src_enc_entropy_encode_rs_399_9_3eb0a8::main
             at ./src/main.rs:4:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
