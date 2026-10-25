# printpdf/src/image.rs:1091:10

- crate: printpdf 0.12.8
- kind: panic
- location: `printpdf/src/image.rs:1091:10`
- source: https://docs.rs/crate/printpdf/0.12.8/source/src/image.rs#1091
- last API call: `printpdf::image_types::RawImage::optimize`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
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
    let mut raw_image = RawImage::empty(1000000000000000000usize, 10000usize, RawImageFormat::R8);
    let _ = raw_image.optimize(&ImageOptimizationOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_printpdf_src_image_rs_1091_10_cdad29`).

## Output

```
thread 'main' (254465) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:1091:10:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <printpdf::image_types::RawImage>::estimate_size_bytes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:1091:10
   4: <printpdf::image_types::RawImage>::optimize
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:630:37
   5: repro_printpdf_src_image_rs_1091_10_cdad29::main
             at ./src/main.rs:7:23
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
