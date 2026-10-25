# out of memory in printpdf::image_types::RawImage::apply_dithering

- crate: printpdf 0.12.8
- kind: out of memory
- last API call: `printpdf::image_types::RawImage::apply_dithering`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling printpdf::image_types::RawImage::apply_dithering
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(0usize, 1000000000usize, RawImageFormat::RG8);
    let _ = raw_image.apply_dithering();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_printpdf_image_types_rawimage_apply_ditheri_86bd8c`).

## Output

```
memory allocation of 24000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<alloc::vec::Vec<i16>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<alloc::vec::Vec<i16>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<alloc::vec::Vec<i16>>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <printpdf::image_types::RawImage>::apply_dithering
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:941:32
  10: <printpdf::image_types::RawImage>::apply_dithering
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/image.rs:1022:26
  11: repro_out_of_memory_in_printpdf_image_types_rawimage_apply_ditheri_86bd8c::main
             at ./src/main.rs:6:23
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
