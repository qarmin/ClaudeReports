# image/src/imageops/sample.rs:578:13

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:578:13`
- last API call: `image::DynamicImage::thumbnail`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8193409602687020826

## Message

```
attempt to add with overflow
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
    let dynamic_image = DynamicImage::from(DynamicImage::new_luma8(1u32, 44917274u32).into_rgba16());
    let _ = dynamic_image.thumbnail(0u32, 1u32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_578_13_2bdd45`).

## Output

```
thread 'main' (1624565) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:578:13:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u32 as core::ops::arith::AddAssign>::add_assign
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:790:51
   4: <image::imageops::sample::ThumbnailSum<u16>>::add_pixel::<image::color::Rgba<u16>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:578:13
   5: image::imageops::sample::thumbnail_sample_block::<image::images::buffer::ImageBuffer<image::color::Rgba<u16>, alloc::vec::Vec<u16>>, image::color::Rgba<u16>, u16>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:712:21
   6: image::imageops::sample::thumbnail::<image::images::buffer::ImageBuffer<image::color::Rgba<u16>, alloc::vec::Vec<u16>>, image::color::Rgba<u16>, u16>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:629:17
   7: <image::images::dynimage::DynamicImage>::thumbnail_exact
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1176:38
   8: <image::images::dynimage::DynamicImage>::thumbnail
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1162:14
   9: repro_image_src_imageops_sample_rs_578_13_2bdd45::main
             at ./src/main.rs:5:27
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
