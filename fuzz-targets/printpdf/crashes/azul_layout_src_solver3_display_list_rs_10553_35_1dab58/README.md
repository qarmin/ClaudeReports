# azul-layout/src/solver3/display_list.rs:10553:35

- crate: printpdf 0.12.8
- kind: panic
- location: `azul-layout/src/solver3/display_list.rs:10553:35`
- source: https://docs.rs/crate/azul-layout/0.0.16/source/src/solver3/display_list.rs#10553
- last API call: `printpdf::html::add_xml_to_document`
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
use printpdf::PdfDocument;
use printpdf::deserialize::PdfParseOptions;
use printpdf::html::XmlRenderOptions;

fn main() {
    let mut pdf_document = PdfDocument::parse(&include_bytes!("data_0.bin").to_vec(), &PdfParseOptions::default(), &mut vec![]).unwrap();
    let _ = printpdf::html::add_xml_to_document(&mut pdf_document, "<svg xs=\"vg\" width=\"1e9\" height=\"55\" vx=\"0N\"><circle cx=\"1\" r=\"0.6\"/></svg>", &XmlRenderOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_azul_layout_src_solver3_display_list_rs_10553_35_1dab58`).

## Output

```
[azul][displaylist] dropping a TextLayout with FontHash::invalid() — its font never resolved, so no renderer would have drawn it
thread 'main' (254688) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:10553:35:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: azul_layout::solver3::display_list::rasterize_svg_clip_to_r8
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:10553:35
   4: <azul_layout::solver3::display_list::DisplayListGenerator<azul_css::props::basic::font::FontRef>>::push_image_mask_clip
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:5129:17
   5: <azul_layout::solver3::display_list::DisplayListGenerator<azul_css::props::basic::font::FontRef>>::paint_in_flow_descendants
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:4764:50
   6: <azul_layout::solver3::display_list::DisplayListGenerator<azul_css::props::basic::font::FontRef>>::paint_in_flow_descendants
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:4775:18
   7: <azul_layout::solver3::display_list::DisplayListGenerator<azul_css::props::basic::font::FontRef>>::paint_in_flow_descendants
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:4775:18
   8: <azul_layout::solver3::display_list::DisplayListGenerator<azul_css::props::basic::font::FontRef>>::generate_for_stacking_context
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:4552:14
   9: azul_layout::solver3::display_list::generate_display_list_impl::<azul_css::props::basic::font::FontRef>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:3163:15
  10: azul_layout::solver3::display_list::generate_display_list::<azul_css::props::basic::font::FontRef>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/display_list.rs:3048:5
  11: azul_layout::solver3::paged_layout::layout_document_paged_impl::<azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/paged_layout.rs:534:29
  12: azul_layout::solver3::paged_layout::layout_document_paged_v2::<azul_css::props::basic::font::FontRef, printpdf::html::xml_to_pdf_pages::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/solver3/paged_layout.rs:227:5
  13: printpdf::html::xml_to_pdf_pages
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/html/mod.rs:589:31
  14: printpdf::html::add_xml_to_document
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/html/mod.rs:1086:11
  15: repro_azul_layout_src_solver3_display_list_rs_10553_35_1dab58::main
             at ./src/main.rs:7:13
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
