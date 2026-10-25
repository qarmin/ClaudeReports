# image/src/codecs/hdr/encoder.rs:63:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/hdr/encoder.rs:63:9`
- last API call: `image::codecs::hdr::HdrEncoder<Vec<u8>>::write_image`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7391001547143732357

## Message

```
not enough pixels provided
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::ExtendedColorType;

fn main() {
    let _ = <image::codecs::hdr::HdrEncoder<Vec<u8>> as image::ImageEncoder>::write_image(<image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(vec![]), &vec![], 1u32, 1u32, ExtendedColorType::Rgb32F);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_hdr_encoder_rs_63_9_ac93cd`).

## Output

```
thread 'main' (267389) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:63:9:
not enough pixels provided
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode_pixels::<core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u8>, <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>> as image::io::encoder::ImageEncoder>::write_image::{closure#0}>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:63:9
   3: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>> as image::io::encoder::ImageEncoder>::write_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:30:22
   4: repro_image_src_codecs_hdr_encoder_rs_63_9_ac93cd::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
