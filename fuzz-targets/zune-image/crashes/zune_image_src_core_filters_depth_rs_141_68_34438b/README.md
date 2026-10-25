# zune-image/src/core_filters/depth.rs:141:68

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/depth.rs:141:68`
- last API call: `zune_image::core_filters::depth::Depth::execute_impl`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3083037189422960871

## Message

```
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0x1f814bf16f812452b283244dea52bee6). This indicates you are converting a channel
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
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::channel::MIN_ALIGNMENT;
use zune_image::codecs::qoi::QoiEncoder;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let usize_v = MIN_ALIGNMENT;
    let mut image = Image::new(vec![Channel::new_with_bit_type(usize_v, BitType::U16)], BitDepth::Float32, 0usize, 0usize, ColorSpace::Luma);
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl(&Depth::new(<zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait>::default_depth(&QoiEncoder::default(), BitDepth::Float32)), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_depth_rs_141_68_34438b`).

## Output

```
thread 'main' (1820067) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:141:68:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14) from expected TypeId(0x1f814bf16f812452b283244dea52bee6). This indicates you are converting a channel
             to a type it wasn't instantiated with
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&[f32], zune_image::channel::ChannelErrors>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/depth.rs:141:68
   5: repro_zune_image_src_core_filters_depth_rs_141_68_34438b::main
             at ./src/main.rs:13:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
