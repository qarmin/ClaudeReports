# zune-image/src/core_filters/depth.rs:167:67

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/depth.rs:167:67`
- last API call: `zune_image::core_filters::colorspace::ColorspaceConv::execute`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 11670860421714670319

## Message

```
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
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
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::qoi::QoiEncoder;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::fill::<f32>(0.0f32, ColorSpace::HSL, 0usize, 0usize);
    image.set_depth(<zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait>::default_depth(&QoiEncoder::default(), BitDepth::Float32));
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute(&ColorspaceConv::new(ColorSpace::CMYK), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_depth_rs_167_67_44bfdc`).

## Output

```
thread 'main' (3224544) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:167:67:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0xcfb1622e7734f8af272d8d182905dc25). This indicates you are converting a channel
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
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:167:67
   5: <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:199:14
   6: <zune_image::image::Image>::convert_depth
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:698:24
   7: zune_image::core_filters::colorspace::conversion_functions::convert_hsl_to_rgb
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:329:11
   8: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace.rs:162:17
   9: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:199:14
  10: repro_zune_image_src_core_filters_depth_rs_167_67_44bfdc::main
             at ./src/main.rs:10:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
