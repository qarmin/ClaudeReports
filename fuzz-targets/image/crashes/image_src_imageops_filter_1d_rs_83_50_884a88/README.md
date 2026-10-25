# image/src/imageops/filter_1d.rs:83:50

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/filter_1d.rs:83:50`
- last API call: `image::DynamicImage::blur_advanced`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

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
use image::imageops::GaussianBlurParameters;

fn main() {
    let _ = DynamicImage::default().blur_advanced(GaussianBlurParameters::SMOOTHING_5);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_filter_1d_rs_83_50_884a88`).

## Output

```
thread 'main' (3295128) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:83:50:
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
   4: <core::option::Option<&[u8; 4]>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: image::imageops::filter_1d::make_arena_row::<u8, 4>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:83:50
   6: image::imageops::filter_1d::filter_2d_separable_ring_queue::<u8, u32, 4>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:529:5
   7: image::imageops::filter_1d::filter_2d_separable::<u8, f32, u32, 4>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:674:16
   8: image::imageops::filter_1d::filter_2d_sep_rgba
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/filter_1d.rs:818:5
   9: image::imageops::sample::gaussian_blur_dyn_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1376:13
  10: <image::images::dynimage::DynamicImage>::blur_advanced
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1244:9
  11: repro_image_src_imageops_filter_1d_rs_83_50_884a88::main
             at ./src/main.rs:5:37
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
