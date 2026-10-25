# zune-inflate/src/encoder.rs:203:44

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-inflate/src/encoder.rs:203:44`
- last API call: `zune_inflate::DeflateEncoder::encode_zlib`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 2 out of range for slice of length 1
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
    let vec = vec![66u8];
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    deflate_encoder.encode_deflate();
    let _ = deflate_encoder.encode_zlib();
}
```

Run with `cargo run` in this directory (package `repro_zune_inflate_src_encoder_rs_203_44_942e3a`).

## Output

```
thread 'main' (3372534) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:203:44:
range end index 2 out of range for slice of length 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[u8] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <zune_inflate::encoder::DeflateEncoder>::encode_no_compression
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:203:44
   6: <zune_inflate::encoder::DeflateEncoder>::encode_deflate
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:217:22
   7: <zune_inflate::encoder::DeflateEncoder>::encode_zlib
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-inflate/src/encoder.rs:229:14
   8: repro_zune_inflate_src_encoder_rs_203_44_942e3a::main
             at ./src/main.rs:7:29
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
