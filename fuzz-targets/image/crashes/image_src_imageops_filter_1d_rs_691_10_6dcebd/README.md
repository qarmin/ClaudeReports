# image/src/imageops/filter_1d.rs:691:10

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/filter_1d.rs:691:10`
- last API call: `image::DynamicImage::unsharpen`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
chunk size must be non-zero
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
    let dynamic_image = DynamicImage::from(<image::ImageBuffer<image::Rgba<f32>, Vec<f32>> as ::core::default::Default>::default());
    let dynamic_image2 = dynamic_image.flipv();
    let _ = dynamic_image2.unsharpen(65536.0f32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_filter_1d_rs_691_10_6dcebd`).

## Output

```
thread 'main' (3295707) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:691:10:
chunk size must be non-zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[f32]>::chunks_exact_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1290:9
   3: image::imageops::filter_1d::filter_2d_separable::<f32, f32, f32, 4>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:691:10
   4: image::imageops::filter_1d::filter_2d_sep_rgba_f32
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:882:5
   5: image::imageops::sample::gaussian_blur_indirect_impl::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, 4>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1599:13
   6: image::imageops::sample::gaussian_blur_indirect::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1515:14
   7: image::imageops::sample::blur_advanced::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1069:5
   8: image::imageops::sample::unsharpen::<image::images::buffer::ImageBuffer<image::color::Rgba<f32>, alloc::vec::Vec<f32>>, image::color::Rgba<f32>, f32>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1638:19
   9: <image::images::dynimage::DynamicImage>::unsharpen
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1278:38
  10: repro_image_src_imageops_filter_1d_rs_691_10_6dcebd::main
             at ./src/main.rs:6:28
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
