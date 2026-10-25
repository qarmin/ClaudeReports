# zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5`
- last API call: `zune_image::core_filters::colorspace::ColorspaceConv::execute`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13037977160648300288

## Message

```
assertion `left != right` failed
  left: Unknown
 right: Unknown
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
use zune_image::codecs::ppm::PPMEncoder;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new_frames(vec![], <zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait>::default_depth(&PPMEncoder::new(), BitDepth::Unknown), 0usize, 0usize, ColorSpace::HSL);
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute(&ColorspaceConv::new(ColorSpace::LumaA), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_colorspace_conversion_functions__7f73b0`).

## Output

```
thread 'main' (405193) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5:
assertion `left != right` failed
  left: Unknown
 right: Unknown
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<zune_core::colorspace::ColorSpace, zune_core::colorspace::ColorSpace>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: zune_image::core_filters::colorspace::conversion_functions::convert_rgb_to_grayscale
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:164:5
   5: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace.rs:77:38
   6: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:199:14
   7: <zune_image::image::Image>::convert_color
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:691:33
   8: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace.rs:165:23
   9: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:199:14
  10: repro_zune_image_src_core_filters_colorspace_conversion_functions__7f73b0::main
             at ./src/main.rs:9:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
