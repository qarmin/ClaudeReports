# zune-inflate/src/encoder.rs:165:24

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-inflate/src/encoder.rs:165:24`
- last API call: `zune_inflate::DeflateEncoder::encode_deflate`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 11
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-inflate = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_inflate::DeflateEncoder;

fn main() {
    let vec = vec![];
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    let _ = deflate_encoder.encode_zlib();
    deflate_encoder.encode_deflate();
}
```

Run with `cargo run` in this directory (package `repro_zune_inflate_src_encoder_rs_165_24_b4c163`).

## Output

```
thread 'main' (3374536) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:165:24:
index out of bounds: the len is 0 but the index is 11
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:245:14
   4: <[u8] as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<u8> as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <zune_inflate::encoder::DeflateEncoder>::encode_no_compression
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:165:24
   7: <zune_inflate::encoder::DeflateEncoder>::encode_deflate
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:217:22
   8: repro_zune_inflate_src_encoder_rs_165_24_b4c163::main
             at ./src/main.rs:7:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
