# image/src/imageops/colorops.rs:170:47

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/colorops.rs:170:47`
- last API call: `image::DynamicImage::brighten`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13037096078539116402

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
    let dynamic_image = DynamicImage::ImageRgba32F(DynamicImage::new_luma16(1u32, 1u32).to_rgba32f());
    let dynamic_image2 = dynamic_image.filter3x3(&[0.0f32, 0.0f32, f32::NAN, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]);
    let _ = dynamic_image2.brighten(0i32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_colorops_rs_170_47_dfc6fd`).

## Output

```
thread 'main' (362067) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:170:47:
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
   4: <core::option::Option<i32>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: image::imageops::colorops::brighten::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, image::color::Rgba<f32>, f32>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:170:47
   6: <image::color::Rgba<f32> as image::traits::Pixel>::apply_with_alpha::<image::imageops::colorops::brighten<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, image::color::Rgba<f32>, f32>::{closure#0}, image::imageops::colorops::brighten<image::images::buffer::Im... (+101 chars)
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:457:18
   7: <image::color::Rgba<f32> as image::traits::Pixel>::map_with_alpha::<image::imageops::colorops::brighten<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, image::color::Rgba<f32>, f32>::{closure#0}, image::imageops::colorops::brighten<image::images::buffer::Imag... (+99 chars)
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/traits.rs:531:14
   8: image::imageops::colorops::brighten::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, image::color::Rgba<f32>, f32>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:168:23
   9: <image::images::dynimage::DynamicImage>::brighten
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1318:38
  10: repro_image_src_imageops_colorops_rs_170_47_dfc6fd::main
             at ./src/main.rs:6:28
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
