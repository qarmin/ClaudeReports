# out of memory in egui::text_selection::TextCursorState::range

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: out of memory
- last API call: `egui::text_selection::TextCursorState::range`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 17747522224903900279

## Message

```
process ended abnormally (out of memory) while calling egui::text_selection::TextCursorState::range
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_demo_lib = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Style;
use egui::text_selection::CCursorRange;
use egui::text_selection::TextCursorState;
use egui_demo_lib::easy_mark::MemoizedEasymarkHighlighter;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let text_cursor_state = TextCursorState::from(CCursorRange::default());
    let _ = text_cursor_state.range(&Fonts::new(TextOptions::default(), FontDefinitions::empty()).layout_uncached(65536.0f32, std::sync::Arc::new(MemoizedEasymarkHighlighter::default().highlight(&Style::default(), "a"))));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_egui_text_selection_textcursorstate_range_01f9f2`).

## Output

```
memory allocation of 1016189943808 bytes failed
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
  24: <epaint::text::fonts::Fonts>::layout_uncached_for_viewport
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:291:9
  25: <epaint::text::fonts::Fonts>::layout_uncached
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:277:14
  26: repro_out_of_memory_in_egui_text_selection_textcursorstate_range_01f9f2::main
             at ./src/main.rs:11:99
  27: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
