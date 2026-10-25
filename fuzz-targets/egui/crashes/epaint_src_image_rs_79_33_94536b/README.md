# epaint/src/image.rs:79:33

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/image.rs:79:33`
- last API call: `epaint::Fonts::layout_uncached`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 35718509433537600

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_extras = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Style;
use egui::Theme;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let _ = Fonts::new(TextOptions::default(), FontDefinitions::empty()).layout_uncached(35184372000000.0f32, std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Theme::Light.default_style(), &CodeTheme::from_memory(&Context::default(), &Style::default()), "Z", "")));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_image_rs_79_33_94536b`).

## Output

```
thread 'main' (43691) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:79:33:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <epaint::image::ColorImage>::filled
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:79:33
   4: epaint::text::fonts::synthetic_tofu
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:870:21
   5: <epaint::text::fonts::FontsImpl>::new::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:524:61
   6: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster::<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:352:36
   7: core::iter::traits::iterator::Iterator::find_map::check::<&epaint::text::glyph_rasterizer::GlyphRasterizer, epaint::text::glyph_rasterizer::RasterizedGlyph, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer... (+29 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2994:32
   8: core::iter::adapters::filter::filter_try_fold::<&epaint::text::glyph_rasterizer::GlyphRasterizer, (), core::ops::control_flow::ControlFlow<epaint::text::glyph_rasterizer::RasterizedGlyph>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::g... (+384 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter.rs:86:44
   9: <core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::adapters::filter::filter_try_fold<&epaint::text::glyph_rasterizer::GlyphRasterizer, (), core::ops::control_flow::ControlFlow<epaint::text::g... (+612 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2490:21
  10: <core::iter::adapters::filter::Filter<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#0}> as core::iter::trai... (+449 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/filter.rs:157:19
  11: <core::iter::adapters::filter::Filter<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>, <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>::{closure#0}> as core::iter::trai... (+238 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3000:14
  12: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_raster::<core::iter::sources::once::Once<&epaint::text::glyph_rasterizer::GlyphRasterizer>>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:352:14
  13: <epaint::text::fonts::FontsImpl>::fontless_cluster
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:837:14
  14: epaint::text::text_layout::segment_into_runs
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:1538:32
  15: epaint::text::text_layout::layout_section
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:572:9
  16: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:116:28
  17: <epaint::text::fonts::Fonts>::layout_uncached_for_viewport
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:291:9
  18: <epaint::text::fonts::Fonts>::layout_uncached
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:277:14
  19: repro_epaint_src_image_rs_79_33_94536b::main
             at ./src/main.rs:10:74
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
