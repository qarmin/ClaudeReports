# zune-core/src/bit_depth.rs:171:30

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-core/src/bit_depth.rs:171:30`
- last API call: `zune_image::codecs::farbfeld::FarbFeldEncoder::encode_to_result`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2910338399052617372

## Message

```
Unknown bit type
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
use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize, ColorSpace::YCbCr));
}
```

Run with `cargo run` in this directory (package `repro_zune_core_src_bit_depth_rs_171_30_bc4870`).

## Output

```
thread 'main' (269786) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/bit_depth.rs:171:30:
Unknown bit type
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <zune_core::bit_depth::BitDepth>::size_of
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/bit_depth.rs:171:30
   3: zune_image::traits::confirm_invariants
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:270:41
   4: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:344:9
   5: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:420:25
   6: repro_zune_core_src_bit_depth_rs_171_30_bc4870::main
             at ./src/main.rs:8:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
