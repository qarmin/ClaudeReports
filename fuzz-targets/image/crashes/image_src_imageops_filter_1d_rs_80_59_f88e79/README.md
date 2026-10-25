# image/src/imageops/filter_1d.rs:80:59

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/filter_1d.rs:80:59`
- last API call: `image::DynamicImage::blur_advanced`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 1 out of range for slice of length 0
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
use image::imageops::GaussianBlurParameters;
use image::math::Rect;

fn main() {
    let dynamic_image = DynamicImage::new_rgb32f(10u32, 0u32);
    let dynamic_image2 = dynamic_image.crop(Rect { x: 1u32, y: 0u32, width: 1u32, height: 1u32 });
    let _ = dynamic_image2.blur_advanced(GaussianBlurParameters::SMOOTHING_3);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_filter_1d_rs_80_59_f88e79`).

## Output

```
thread 'main' (3295045) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:80:59:
range end index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[[f32; 3]]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[[f32; 3]]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:503:23
   5: <[[f32; 3]] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: image::imageops::filter_1d::make_arena_row::<f32, 3>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:80:59
   7: image::imageops::filter_1d::filter_2d_separable_ring_queue::<f32, f32, 3>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:529:5
   8: image::imageops::filter_1d::filter_2d_separable::<f32, f32, f32, 3>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:674:16
   9: image::imageops::filter_1d::filter_2d_sep_rgb_f32
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:866:5
  10: image::imageops::sample::gaussian_blur_dyn_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1474:13
  11: <image::images::dynimage::DynamicImage>::blur_advanced
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1244:9
  12: repro_image_src_imageops_filter_1d_rs_80_59_f88e79::main
             at ./src/main.rs:8:28
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
