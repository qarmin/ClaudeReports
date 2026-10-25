# image/src/imageops/sample.rs:962:62

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:962:62`
- last API call: `image::DynamicImage::filter3x3`
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

fn main() {
    let _ = DynamicImage::new_luma16(1u32, 1u32).filter3x3(&[0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, f32::NEG_INFINITY]);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_962_62_f1c465`).

## Output

```
thread 'main' (3295363) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:962:62:
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
   4: <core::option::Option<u16>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: image::imageops::sample::filter3x3::<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:962:62
   6: <core::ops::try_trait::NeverShortCircuit<u16>>::wrap_mut_1::<f32, image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/try_trait.rs:415:42
   7: <&mut core::array::drain::Drain<f32, <core::ops::try_trait::NeverShortCircuit<u16>>::wrap_mut_1<f32, image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}>::{closure#0}> as core::ops::function::FnMut<(usize,)>>::call_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/drain.rs:97:13
   8: core::array::try_from_fn_erased::<core::ops::try_trait::NeverShortCircuit<u16>, &mut core::array::drain::Drain<f32, <core::ops::try_trait::NeverShortCircuit<u16>>::wrap_mut_1<f32, image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}>::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:913:20
   9: core::array::try_from_fn::<core::ops::try_trait::NeverShortCircuit<u16>, 4, &mut core::array::drain::Drain<f32, <core::ops::try_trait::NeverShortCircuit<u16>>::wrap_mut_1<f32, image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}>::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:155:11
  10: <[f32; 4]>::try_map::<core::ops::try_trait::NeverShortCircuit<u16>, <core::ops::try_trait::NeverShortCircuit<u16>>::wrap_mut_1<f32, image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:644:9
  11: <[f32; 4]>::map::<image::imageops::sample::filter3x3<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>::{closure#1}, u16>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:598:14
  12: image::imageops::sample::filter3x3::<image::images::buffer::ImageBuffer<image::color::Luma<u16>, alloc::vec::Vec<u16>>, image::color::Luma<u16>, u16>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:962:18
  13: <image::images::dynimage::DynamicImage>::filter3x3
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1296:38
  14: repro_image_src_imageops_sample_rs_962_62_f1c465::main
             at ./src/main.rs:4:50
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
