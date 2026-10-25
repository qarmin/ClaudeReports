# image/src/io/encoder.rs:115:17

- crate: printpdf 0.12.8
- kind: panic
- location: `image/src/io/encoder.rs:115:17`
- source: https://docs.rs/crate/image/0.25.10/source/src/io/encoder.rs#115
- last API call: `printpdf::image_types::RawImage::encode_to_bytes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: Invalid buffer length: expected 0 got 1 for 0x0 image
  left: 0
 right: 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::image_types::OutputImageFormat;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageData;
use printpdf::image_types::RawImageFormat;

fn main() {
    let raw_image = RawImage { pixels: RawImageData::U8(vec![119u8]), width: 0usize, height: 0usize, data_format: RawImageFormat::RG8, tag: vec![] };
    let _ = raw_image.encode_to_bytes(&[OutputImageFormat::Pnm]);
}
```

Run with `cargo run` in this directory (package `repro_image_src_io_encoder_rs_115_17_a81cb6`).

## Output

```
thread 'main' (255948) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/io/encoder.rs:115:17:
assertion `left == right` failed: Invalid buffer length: expected 0 got 1 for 0x0 image
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<u64, u64>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <image::codecs::pnm::encoder::PnmEncoder<&mut &mut core::io::cursor::Cursor<&mut alloc::vec::Vec<u8>>> as image::io::encoder::ImageEncoder>::write_image
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/codecs/pnm/encoder.rs:347:9
   5: <image::codecs::pnm::encoder::PnmEncoder<&mut &mut core::io::cursor::Cursor<&mut alloc::vec::Vec<u8>>> as image::io::encoder::ImageEncoderBoxed>::write_image
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/io/encoder.rs:115:17
   6: <image::images::dynimage::DynamicImage>::write_with_encoder_impl
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/images/dynimage.rs:1353:17
   7: <image::images::dynimage::DynamicImage>::write_to::<&mut core::io::cursor::Cursor<&mut alloc::vec::Vec<u8>>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.10/src/images/dynimage.rs:1373:14
   8: <printpdf::image_types::RawImage>::encode_to_bytes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:584:18
   9: repro_image_src_io_encoder_rs_115_17_a81cb6::main
             at ./src/main.rs:8:23
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
