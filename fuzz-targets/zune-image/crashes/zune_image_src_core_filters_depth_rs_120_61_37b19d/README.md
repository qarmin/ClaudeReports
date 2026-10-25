# zune-image/src/core_filters/depth.rs:120:61

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/depth.rs:120:61`
- last API call: `zune_image::core_filters::depth::Depth::execute_impl`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9507283259821186323

## Message

```
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
             to a type it wasn't instantiated with
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
use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new(vec![Channel::new_with_bit_type(0usize, BitType::U16), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Eight, 0usize, 0usize, <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>> as zune_image::traits::DecoderTrait>::out_colorspace(&<zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 0u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 0u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 0u8, 0u8])).unwrap()));
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl(&Depth::new(BitDepth::Sixteen), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_depth_rs_120_61_37b19d`).

## Output

```
thread 'main' (518446) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:120:61:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
             to a type it wasn't instantiated with
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&[u8], zune_image::channel::ChannelErrors>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:120:61
   5: repro_zune_image_src_core_filters_depth_rs_120_61_37b19d::main
             at ./src/main.rs:9:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
