# out of memory in printpdf::PdfDocument::add_xobject

- crate: printpdf 0.12.8
- kind: out of memory
- last API call: `printpdf::PdfDocument::add_xobject`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling printpdf::PdfDocument::add_xobject
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::PdfDocument;
use printpdf::svg::Svg;

fn main() {
    let mut pdf_document = PdfDocument::new("");
    let _ = pdf_document.add_xobject(&Svg::parse("<svg xmlns=\"http://www.w3.org/2000/svg\" x=\"0\"><defs>><filter id=\"f\"><feGaussianBlur /></filter></defs><g mask=\"ly\" filter=\"url(#f)\"><g mask=\"urlf)\"><path d=\"M82 1e9 L131 2 d\"/><text>He😀</text></g></g></svg>", &mut vec![]).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_printpdf_pdfdocument_add_xobject_106f2b`).

## Output

```
memory allocation of 633600000000 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <tiny_skia::pixmap::Pixmap>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-skia-0.11.4/src/pixmap.rs:51:19
  10: svg2pdf::render::filter::render
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/filter.rs:30:22
  11: svg2pdf::render::group::render
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/group.rs:28:16
  12: <usvg::tree::Node as svg2pdf::render::Render>::render
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/mod.rs:98:17
  13: svg2pdf::render::group::create_to_stream
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/group.rs:140:15
  14: svg2pdf::render::group::render
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/group.rs:72:9
  15: svg2pdf::render::tree_to_stream
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/render/mod.rs:39:5
  16: svg2pdf::to_pdf
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/svg2pdf-0.13.0/src/lib.rs:230:5
  17: <printpdf::svg::Svg>::parse_with_fonts
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/svg.rs:120:25
  18: <printpdf::svg::Svg>::parse
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/svg.rs:28:9
  19: repro_out_of_memory_in_printpdf_pdfdocument_add_xobject_106f2b::main
             at ./src/main.rs:6:39
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
