# out of memory in epaint::Fonts::layout_uncached_for_viewport

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: out of memory
- last API call: `epaint::Fonts::layout_uncached_for_viewport`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3698376831730871658

## Message

```
process ended abnormally (out of memory) while calling epaint::Fonts::layout_uncached_for_viewport
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec", features = ["_override_unity", "bytemuck", "callstack", "cint", "color-hex", "default_fonts", "document-features", "experimental", "mint", "monochrome_emoji_fonts", "persistence", "rayon", "ron", "serde", "unity", "web_file"] }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::ViewportKey;

fn main() {
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
    let _ = fonts.layout_uncached_for_viewport(10000000000.0f32, ViewportKey::default(), std::sync::Arc::new(LayoutJob::simple_singleline("\"".to_string(), FontId::default(), Color32::from_rgb_additive(0u8, 0u8, 0u8))));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_epaint_fonts_layout_uncached_for_viewport_3383d6`).

## Output

```
memory allocation of 12237977088 bytes failed
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
   6: <alloc::raw_vec::RawVec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <color::rgba8::PremulRgba8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:14:21
   9: alloc::vec::from_elem::<color::rgba8::PremulRgba8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  10: <vello_common::pixmap::Pixmap>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.2.0/src/pixmap.rs:82:19
  11: <epaint::text::font_face::FontCell>::rasterize_glyph
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/font_face.rs:159:24
  12: <epaint::text::font_face::FontFace>::rasterize_glyph
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/font_face.rs:673:14
  13: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_outline::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:305:18
  14: <std::collections::hash::map::Entry<epaint::text::glyph_atlas::OutlineGlyphKey, epaint::text::glyph_atlas::GlyphAllocation>>::or_insert_with::<<epaint::text::glyph_atlas::GlyphAtlas>::allocate_outline::{closure#0}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2540:55
  15: <std::collections::hash::map::Entry<epaint::text::glyph_atlas::OutlineGlyphKey, epaint::text::glyph_atlas::GlyphAllocation>>::or_try_insert_with::<<std::collections::hash::map::Entry<epaint::text::glyph_atlas::OutlineGlyphKey, epaint::text::glyph_atlas::GlyphAllocation>>::or_insert_with<<epain... (+82 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2575:46
  16: <std::collections::hash::map::Entry<epaint::text::glyph_atlas::OutlineGlyphKey, epaint::text::glyph_atlas::GlyphAllocation>>::or_insert_with::<<epaint::text::glyph_atlas::GlyphAtlas>::allocate_outline::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:2540:14
  17: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_outline
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:304:53
  18: <epaint::text::fonts::FontsImpl>::allocate_glyph
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:753:14
  19: epaint::text::text_layout::layout_shaped_run
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:367:23
  20: epaint::text::text_layout::layout_section
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:603:13
  21: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:116:28
  22: <epaint::text::fonts::Fonts>::layout_uncached_for_viewport
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:291:9
  23: repro_out_of_memory_in_epaint_fonts_layout_uncached_for_viewport_3383d6::main
             at ./src/main.rs:11:19
  24: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
