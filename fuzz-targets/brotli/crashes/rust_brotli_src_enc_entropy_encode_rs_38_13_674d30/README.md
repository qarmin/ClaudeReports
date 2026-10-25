# rust-brotli/src/enc/entropy_encode.rs:38:13

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/entropy_encode.rs:38:13`
- last API call: `brotli::enc::entropy_encode::BrotliSetDepth`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 16 but the index is 16
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::entropy_encode::BrotliSetDepth;
use brotli::enc::entropy_encode::HuffmanTree;

fn main() {
    let _ = BrotliSetDepth(0i32, &mut vec![HuffmanTree::default()], &mut vec![], 1000000000i32);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_entropy_encode_rs_38_13_674d30`).

## Output

```
thread 'main' (1255151) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:38:13:
index out of bounds: the len is 16 but the index is 16
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::entropy_encode::BrotliSetDepth
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/entropy_encode.rs:38:13
   4: repro_rust_brotli_src_enc_entropy_encode_rs_38_13_674d30::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
