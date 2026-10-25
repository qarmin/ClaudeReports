# image/src/codecs/hdr/encoder.rs:287:19

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/hdr/encoder.rs:287:19`
- last API call: `image::codecs::hdr::HdrEncoder<Vec<u8>>::encode`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12382442124681211254

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
fn main() {
    let hdr_encoder = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(vec![]);
    let _ = hdr_encoder.encode(&[<image::Rgb<f32> as ::core::convert::From<[f32; 3]>>::from([0.0f32, 0.0f32, f32::INFINITY])], 0usize, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_hdr_encoder_rs_287_19_c9941f`).

## Output

```
thread 'main' (385511) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:287:19:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: image::codecs::hdr::encoder::to_rgbe8
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:287:19
   4: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0}
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:51:50
   5: <&mut <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0} as core::ops::function::FnOnce<(&image::color::Rgb<f32>,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   6: <core::option::Option<&image::color::Rgb<f32>>>::map::<image::codecs::hdr::decoder::Rgbe8Pixel, &mut <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
   7: <core::iter::adapters::map::Map<core::slice::iter::Iter<image::color::Rgb<f32>>, <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   8: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode_pixels::<core::iter::adapters::map::Map<core::slice::iter::Iter<image::color::Rgb<f32>>, <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0}>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:76:26
   9: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:51:14
  10: repro_image_src_codecs_hdr_encoder_rs_287_19_c9941f::main
             at ./src/main.rs:3:25
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
