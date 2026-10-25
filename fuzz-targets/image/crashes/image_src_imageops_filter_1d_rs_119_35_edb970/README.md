# image/src/imageops/filter_1d.rs:119:35

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/filter_1d.rs:119:35`
- last API call: `image::DynamicImage::blur`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16832082036349918134

## Message

```
range end index 3 out of range for slice of length 0
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
    let dynamic_image = DynamicImage::from(<image::ImageBuffer<image::Rgb<f32>, Vec<f32>>>::from_raw(1u32, 0u32, vec![]).unwrap());
    let _ = dynamic_image.blur(409.0f32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_filter_1d_rs_119_35_edb970`).

## Output

```
thread 'main' (345991) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:119:35:
range end index 3 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[f32]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[f32]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:503:23
   5: <[f32] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: image::imageops::filter_1d::make_columns_arenas::<f32, 3>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:119:35
   7: image::imageops::filter_1d::filter_2d_separable::<f32, f32, f32, 3>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:728:9
   8: image::imageops::filter_1d::filter_2d_sep_rgb_f32
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:866:5
   9: image::imageops::sample::gaussian_blur_dyn_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1474:13
  10: <image::images::dynimage::DynamicImage>::blur
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1227:9
  11: repro_image_src_imageops_filter_1d_rs_119_35_edb970::main
             at ./src/main.rs:5:27
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
