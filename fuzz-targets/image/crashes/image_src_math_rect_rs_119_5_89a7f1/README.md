# image/src/math/rect.rs:119:5

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/math/rect.rs:119:5`
- last API call: `image::ImageBuffer<image::Luma<u16>, Vec<u16>>::sub_image`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2493493343294049229

## Message

```
The rectangle Rect { x: 0, y: 0, width: 0, height: 1 } is out of bounds for the image with dimensions 0×0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::math::Rect;

fn main() {
    let mut image_buffer = <image::ImageBuffer<image::Luma<u16>, Vec<u16>> as ::core::default::Default>::default();
    let _ = <image::ImageBuffer<image::Luma<u16>, Vec<u16>> as image::GenericImage>::sub_image(&mut image_buffer, Rect { x: 0u32, y: 0u32, width: 0u32, height: 1u32 });
}
```

Run with `cargo run` in this directory (package `repro_image_src_math_rect_rs_119_5_89a7f1`).

## Output

```
thread 'main' (355238) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/math/rect.rs:119:5:
The rectangle Rect { x: 0, y: 0, width: 0, height: 1 } is out of bounds for the image with dimensions 0×0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: image::math::rect::panic_out_of_bounds
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/math/rect.rs:119:5
   3: <image::math::rect::Rect>::assert_in_bounds_of::<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/math/rect.rs:94:13
   4: <image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>> as image::images::generic_image::GenericImage>::sub_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/generic_image.rs:362:14
   5: repro_image_src_math_rect_rs_119_5_89a7f1::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
