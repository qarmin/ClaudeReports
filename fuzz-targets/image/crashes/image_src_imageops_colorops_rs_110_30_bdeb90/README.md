# image/src/imageops/colorops.rs:110:30

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/colorops.rs:110:30`
- last API call: `image::DynamicImage::adjust_contrast`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6409381630886233949

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::DynamicImage;

fn main() {
    let _ = DynamicImage::new_luma8(1u32, 1u32).adjust_contrast(f32::NAN);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_colorops_rs_110_30_bdeb90`).

## Output

```
thread 'main' (349385) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:110:30:
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
   4: <core::option::Option<u8>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: image::imageops::colorops::contrast::<image::images::buffer::ImageBuffer<image::color::Luma<u8>, alloc::vec::Vec<u8>>, image::color::Luma<u8>, u8>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:110:30
   6: <image::color::Luma<u8> as image::traits::Pixel>::apply::<image::imageops::colorops::contrast<image::images::buffer::ImageBuffer<image::color::Luma<u8>, alloc::vec::Vec<u8>>, image::color::Luma<u8>, u8>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:450:18
   7: <image::color::Luma<u8> as image::traits::Pixel>::map::<image::imageops::colorops::contrast<image::images::buffer::ImageBuffer<image::color::Luma<u8>, alloc::vec::Vec<u8>>, image::color::Luma<u8>, u8>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/traits.rs:514:14
   8: image::imageops::colorops::contrast::<image::images::buffer::ImageBuffer<image::color::Luma<u8>, alloc::vec::Vec<u8>>, image::color::Luma<u8>, u8>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:104:23
   9: <image::images::dynimage::DynamicImage>::adjust_contrast
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1307:38
  10: repro_image_src_imageops_colorops_rs_110_30_bdeb90::main
             at ./src/main.rs:4:49
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
