# image/src/imageops/colorops.rs:203:35

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/colorops.rs:203:35`
- last API call: `image::imageops::brighten_in_place`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9514434046366906698

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
    let mut dynamic_image = DynamicImage::ImageLumaA8(<image::ImageBuffer<image::LumaA<u8>, Vec<u8>>>::from_raw(1u32, 1u32, vec![77u8, 54u8]).unwrap());
    image::imageops::brighten_in_place::<image::DynamicImage>(&mut dynamic_image, i32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_colorops_rs_203_35_3aad51`).

## Output

```
thread 'main' (2596055) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:203:35:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: image::imageops::colorops::brighten_in_place::<image::images::dynimage::DynamicImage>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:203:35
   4: <image::color::Rgba<u8> as image::traits::Pixel>::apply_with_alpha::<image::imageops::colorops::brighten_in_place<image::images::dynimage::DynamicImage>::{closure#0}, image::imageops::colorops::brighten_in_place<image::images::dynimage::DynamicImage>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:457:18
   5: <image::color::Rgba<u8> as image::traits::Pixel>::map_with_alpha::<image::imageops::colorops::brighten_in_place<image::images::dynimage::DynamicImage>::{closure#0}, image::imageops::colorops::brighten_in_place<image::images::dynimage::DynamicImage>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/traits.rs:531:14
   6: image::imageops::colorops::brighten_in_place::<image::images::dynimage::DynamicImage>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/colorops.rs:200:43
   7: repro_image_src_imageops_colorops_rs_203_35_3aad51::main
             at ./src/main.rs:5:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
