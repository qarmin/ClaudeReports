# zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36`
- last API call: `zune_image::core_filters::colorspace::ColorspaceConv::execute_impl`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 356534818605709221

## Message

```
called `Option::unwrap()` on a `None` value
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
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new(vec![], BitDepth::Float32, 0usize, 1usize, <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>> as zune_image::traits::DecoderTrait>::out_colorspace(&<zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 26u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 13u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 0u8, 16u8])).unwrap()));
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::RGB), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_colorspace_conversion_functions__a1d03b`).

## Output

```
thread 'main' (536573) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<zune_image::channel::Channel>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: zune_image::core_filters::colorspace::conversion_functions::pop_channel
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:399:36
   6: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace.rs:90:36
   7: repro_zune_image_src_core_filters_colorspace_conversion_functions__a1d03b::main
             at ./src/main.rs:8:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
