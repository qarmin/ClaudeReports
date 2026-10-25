# image/src/codecs/gif.rs:582:25

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/gif.rs:582:25`
- last API call: `image::codecs::gif::GifEncoder<Vec<u8>>::encode_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6086665480189262255

## Message

```
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 0
 right: 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::Frame;
use image::codecs::bmp::RowsDecoded;

fn main() {
    let rows_decoded = RowsDecoded::TopDown { rows: 0u32 };
    let u32_v = rows_decoded.rows();
    let mut gif_encoder = <image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]);
    let _ = gif_encoder.encode_frame(Frame::new(<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>::from_raw(u32_v, 1u32, vec![71u8]).unwrap()));
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_gif_rs_582_25_7dec08`).

## Output

```
thread 'main' (576173) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:582:25:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <gif::common::Frame>::from_rgba_speed
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gif-0.14.2/src/common.rs:216:9
   5: <image::codecs::gif::GifEncoder<alloc::vec::Vec<u8>>>::convert_frame
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:582:25
   6: <image::codecs::gif::GifEncoder<alloc::vec::Vec<u8>>>::encode_frame
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:542:26
   7: repro_image_src_codecs_gif_rs_582_25_7dec08::main
             at ./src/main.rs:8:25
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
