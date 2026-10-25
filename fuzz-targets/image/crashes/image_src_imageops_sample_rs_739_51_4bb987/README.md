# image/src/imageops/sample.rs:739:51

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:739:51`
- last API call: `image::DynamicImage::thumbnail`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8851845341803129497

## Message

```
attempt to divide by zero
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
    let _ = DynamicImage::new_luma_a8(10u32, 10000u32).thumbnail(0u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_739_51_4bb987`).

## Output

```
thread 'main' (346282) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:739:51:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u32 as core::ops::arith::Div>::div
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:493:45
   4: image::imageops::sample::thumbnail_sample_block::<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:739:51
   5: <core::ops::try_trait::NeverShortCircuit<u8>>::wrap_mut_1::<u32, image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>::{closure#0}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/try_trait.rs:415:42
   6: <&mut core::array::drain::Drain<u32, <core::ops::try_trait::NeverShortCircuit<u8>>::wrap_mut_1<u32, image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>::{closure#0}>::{closure#0}> as core... (+43 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/drain.rs:97:13
   7: core::array::try_from_fn_erased::<core::ops::try_trait::NeverShortCircuit<u8>, &mut core::array::drain::Drain<u32, <core::ops::try_trait::NeverShortCircuit<u8>>::wrap_mut_1<u32, image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::v... (+71 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:913:20
   8: core::array::try_from_fn::<core::ops::try_trait::NeverShortCircuit<u8>, 4, &mut core::array::drain::Drain<u32, <core::ops::try_trait::NeverShortCircuit<u8>>::wrap_mut_1<u32, image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::... (+67 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:155:11
   9: <[u32; 4]>::try_map::<core::ops::try_trait::NeverShortCircuit<u8>, <core::ops::try_trait::NeverShortCircuit<u8>>::wrap_mut_1<u32, image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>::{clo... (+22 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:644:9
  10: <[u32; 4]>::map::<image::imageops::sample::thumbnail_sample_block<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>::{closure#0}, u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:598:14
  11: image::imageops::sample::thumbnail_sample_block::<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:739:29
  12: image::imageops::sample::thumbnail::<image::images::buffer::ImageBuffer<image::color::LumaA<u8>, alloc::vec::Vec<u8>>, image::color::LumaA<u8>, u8>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:629:17
  13: <image::images::dynimage::DynamicImage>::thumbnail_exact
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1176:38
  14: <image::images::dynimage::DynamicImage>::thumbnail
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1162:14
  15: repro_image_src_imageops_sample_rs_739_51_4bb987::main
             at ./src/main.rs:4:56
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
