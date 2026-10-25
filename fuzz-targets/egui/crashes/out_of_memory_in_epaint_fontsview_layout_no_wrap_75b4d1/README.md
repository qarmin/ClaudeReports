# out of memory in epaint::FontsView::layout_no_wrap

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: out of memory
- last API call: `epaint::FontsView::layout_no_wrap`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2253824730406757484

## Message

```
process ended abnormally (out of memory) while calling epaint::FontsView::layout_no_wrap
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use epaint::FontFamily;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::ViewportKey;

fn main() {
    let color32 = Color32::DARK_GRAY;
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let mut fonts_view = fonts.with_pixels_per_point_for_viewport(721.0f32, ViewportKey::default());
    let _ = fonts_view.layout_no_wrap("a".to_string(), FontId::new(642.0f32, FontFamily::Monospace), color32);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_epaint_fontsview_layout_no_wrap_75b4d1`).

## Output

```
memory allocation of 299963273988 bytes failed
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
   6: <alloc::raw_vec::RawVec<ecolor::color32::Color32>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<ecolor::color32::Color32>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <ecolor::color32::Color32 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:14:21
   9: alloc::vec::from_elem::<ecolor::color32::Color32>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  10: <epaint::image::ColorImage>::filled
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:79:21
  11: epaint::text::fonts::synthetic_tofu
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:870:21
  12: <epaint::text::fonts::FontsImpl>::new::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:524:61
  13: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster::<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:352:36
  14: core::iter::traits::iterator::Iterator::find_map::check::<&epaint::text::glyph_rasterizer::GlyphRasterizer, epaint::text::glyph_rasterizer::RasterizedGlyph, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer... (+29 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2994:32
  15: core::iter::adapters::filter::filter_try_fold::<&epaint::text::glyph_rasterizer::GlyphRasterizer, (), core::ops::control_flow::ControlFlow<epaint::text::glyph_rasterizer::RasterizedGlyph>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::g... (+384 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter.rs:86:44
  16: <core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::adapters::filter::filter_try_fold<&epaint::text::glyph_rasterizer::GlyphRasterizer, (), core::ops::control_flow::ControlFlow<epaint::text::g... (+612 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2490:21
  17: <core::iter::adapters::filter::Filter<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#0}> as core::iter::trai... (+449 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter.rs:157:19
  18: <core::iter::adapters::filter::Filter<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#0}> as core::iter::trai... (+238 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3000:14
  19: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster::<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:352:14
  20: <epaint::text::fonts::FontsImpl>::fontless_cluster
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:837:14
  21: epaint::text::text_layout::segment_into_runs
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:1538:32
  22: epaint::text::text_layout::layout_section
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:572:9
  23: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:116:28
  24: <epaint::text::galley_cache::GalleyCache>::layout_internal
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/galley_cache.rs:112:34
  25: <epaint::text::galley_cache::GalleyCache>::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/galley_cache.rs:134:14
  26: <epaint::text::fonts::FontsView>::layout_job
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:421:27
  27: <epaint::text::fonts::FontsView>::layout_no_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:462:14
  28: repro_out_of_memory_in_epaint_fontsview_layout_no_wrap_75b4d1::main
             at ./src/main.rs:13:24
  29: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
