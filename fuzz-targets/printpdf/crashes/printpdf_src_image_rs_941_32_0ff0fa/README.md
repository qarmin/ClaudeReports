# printpdf/src/image.rs:941:32

- crate: printpdf 0.12.8
- kind: panic
- location: `printpdf/src/image.rs:941:32`
- source: https://docs.rs/crate/printpdf/0.12.8/source/src/image.rs#941
- last API call: `printpdf::image_types::RawImage::optimize`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::image_types::ImageOptimizationOptions;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(0usize, 1000000000000000000usize, RawImageFormat::R16);
    let _ = raw_image.optimize(&ImageOptimizationOptions { quality: None, max_image_size: None, dither_greyscale: Some(true), convert_to_greyscale: None, auto_optimize: None, format: None });
}
```

Run with `cargo run` in this directory (package `repro_printpdf_src_image_rs_941_32_0ff0fa`).

## Output

```
thread 'main' (254588) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   5: <alloc::raw_vec::RawVec<alloc::vec::Vec<i16>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<alloc::vec::Vec<i16>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<alloc::vec::Vec<i16>>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <printpdf::image_types::RawImage>::apply_dithering
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:941:32
   9: <printpdf::image_types::RawImage>::apply_dithering
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:1057:26
  10: <printpdf::image_types::RawImage>::optimize
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:621:18
  11: repro_printpdf_src_image_rs_941_32_0ff0fa::main
             at ./src/main.rs:7:23
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
