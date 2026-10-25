# image/src/imageops/sample.rs:1467:14

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:1467:14`
- last API call: `image::DynamicImage::blur_advanced`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 10745116307008507276

## Message

```
called `Result::unwrap()` on an `Err` value: Limits(LimitError { kind: DimensionError })
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
    let dynamic_image = DynamicImage::ImageLumaA32F(<image::ImageBuffer<image::LumaA<f32>, Vec<f32>>>::from_raw(1u32, 0u32, vec![0.0f32]).unwrap());
    let _ = dynamic_image.blur_advanced(GaussianBlurParameters::SMOOTHING_3);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_1467_14_c2382c`).

## Output

```
thread 'main' (806078) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1467:14:
called `Result::unwrap()` on an `Err` value: Limits(LimitError { kind: DimensionError })
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), image::error::ImageError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: image::imageops::sample::gaussian_blur_dyn_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1467:14
   5: <image::images::dynimage::DynamicImage>::blur_advanced
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1244:9
   6: repro_image_src_imageops_sample_rs_1467_14_c2382c::main
             at ./src/main.rs:6:27
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
