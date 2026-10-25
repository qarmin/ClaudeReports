# image/src/imageops/colorops.rs:143:34

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/colorops.rs:143:34`
- last API call: `image::imageops::contrast_in_place`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4040309414378222398

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
    let mut dynamic_image = DynamicImage::new_luma8(1u32, 1u32);
    image::imageops::contrast_in_place::<image::DynamicImage>(&mut dynamic_image, f32::NAN);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_colorops_rs_143_34_fef311`).

## Output

```
thread 'main' (368070) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:143:34:
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
   5: image::imageops::colorops::contrast_in_place::<image::images::dynimage::DynamicImage>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:143:34
   6: <image::color::Rgba<u8> as image::traits::Pixel>::apply::<image::imageops::colorops::contrast_in_place<image::images::dynimage::DynamicImage>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:450:18
   7: <image::color::Rgba<u8> as image::traits::Pixel>::map::<image::imageops::colorops::contrast_in_place<image::images::dynimage::DynamicImage>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/traits.rs:514:14
   8: image::imageops::colorops::contrast_in_place::<image::images::dynimage::DynamicImage>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:137:43
   9: repro_image_src_imageops_colorops_rs_143_34_fef311::main
             at ./src/main.rs:5:5
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
