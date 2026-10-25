# zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18`
- last API call: `zune_image::core_filters::colorspace::ColorspaceConv::execute_impl`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16059706408411105037

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut image = Image::new(vec![Channel::new_with_bit_type(0usize, BitType::F32), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], ImageMetadata::default().depth(), 0usize, 0usize, ColorSpace::CMYK);
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::Unknown), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_core_filters_colorspace_conversion_functions__2665d4`).

## Output

```
thread 'main' (2473439) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: zune_image::core_filters::colorspace::conversion_functions::convert_cmyk_to_rgb
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace/conversion_functions.rs:287:18
   4: <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/core_filters/colorspace.rs:132:17
   5: repro_zune_image_src_core_filters_colorspace_conversion_functions__2665d4::main
             at ./src/main.rs:10:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
