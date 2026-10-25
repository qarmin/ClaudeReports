# out of memory in printpdf::xobject::XObject::External

- crate: printpdf 0.12.8
- kind: out of memory
- last API call: `printpdf::xobject::XObject::External`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling printpdf::xobject::XObject::External
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::svg::Svg;
use printpdf::xobject::XObject;

fn main() {
    let _ = XObject::External(Svg::parse("<svg xmlns=\"http://www.w3.org/2000/svg\" h=\"1\"><defs><filter id=\"f\"><feGaussianBlur sion=\"2\"/></filter></defs><g mask=\"urply\" filter=\"url(#f)\"><text > 😀</text><polyline points=\"181,108 31,1e9\" s=\"0)\"/>\">></g></svg>", &mut vec![]).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_printpdf_xobject_xobject_external_357324`).

## Output

```
memory allocation of 1943999723520 bytes failed
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
  19: repro_out_of_memory_in_printpdf_xobject_xobject_external_357324::main
             at ./src/main.rs:5:31
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
