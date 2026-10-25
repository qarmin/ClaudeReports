# zune-core/src/options/encoder.rs:121:9

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-core/src/options/encoder.rs:121:9`
- last API call: `zune_image::codecs::hdr::HdrEncoder::encode_to_result`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1883248367180144830

## Message

```
assertion `left != right` failed
  left: 0
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false, features = ["bmp", "farbfeld", "hdr", "heic", "jpeg", "jpeg-xl", "jpeg_2000", "png", "ppm", "psd", "qoi", "webp", "metadata", "serde-support", "threads", "simd", "log"] }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::hdr::HdrEncoder;
use zune_image::image::Image;

fn main() {
    let mut hdr_encoder = HdrEncoder::default();
    let _ = <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut hdr_encoder, &Image::fill::<u16>(1u16, ColorSpace::ARGB, 0usize, 0usize));
}
```

Run with `cargo run` in this directory (package `repro_zune_core_src_options_encoder_rs_121_9_75bd4a`).

## Output

```
thread 'main' (265015) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/options/encoder.rs:121:9:
assertion `left != right` failed
  left: 0
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <zune_core::options::encoder::EncoderOptions>::height
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/options/encoder.rs:121:9
   5: <zune_hdr::encoder::HdrEncoder>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-hdr/src/encoder.rs:133:39
   6: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_inner::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/codecs/hdr.rs:103:14
   7: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:395:18
   8: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:420:25
   9: repro_zune_core_src_options_encoder_rs_121_9_75bd4a::main
             at ./src/main.rs:7:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
