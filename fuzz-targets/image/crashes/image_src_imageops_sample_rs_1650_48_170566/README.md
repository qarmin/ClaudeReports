# image/src/imageops/sample.rs:1650:48

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:1650:48`
- last API call: `image::DynamicImage::unsharpen`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14845193239014242029

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
    let dynamic_image = DynamicImage::new_luma32f(1u32, 1u32);
    let dynamic_image2 = dynamic_image.filter3x3(&[0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, f32::INFINITY, 0.0f32, 0.0f32, 0.0f32]);
    let _ = dynamic_image2.unsharpen(0.0f32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_1650_48_170566`).

## Output

```
thread 'main' (1640404) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1650:48:
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
   5: image::imageops::sample::unsharpen::<image::images::buffer::ImageBuffer<image::color::Luma<f32>, alloc::vec::Vec<f32>>, image::color::Luma<f32>, f32>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1650:48
   6: <image::color::Luma<f32> as image::traits::Pixel>::apply2::<image::imageops::sample::unsharpen<image::images::buffer::ImageBuffer<image::color::Luma<f32>, alloc::vec::Vec<f32>>, image::color::Luma<f32>, f32>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:468:18
   7: <image::color::Luma<f32> as image::traits::Pixel>::map2::<image::imageops::sample::unsharpen<image::images::buffer::ImageBuffer<image::color::Luma<f32>, alloc::vec::Vec<f32>>, image::color::Luma<f32>, f32>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/traits.rs:566:14
   8: image::imageops::sample::unsharpen::<image::images::buffer::ImageBuffer<image::color::Luma<f32>, alloc::vec::Vec<f32>>, image::color::Luma<f32>, f32>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1649:23
   9: <image::images::dynimage::DynamicImage>::unsharpen
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1278:38
  10: repro_image_src_imageops_sample_rs_1650_48_170566::main
             at ./src/main.rs:6:28
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
