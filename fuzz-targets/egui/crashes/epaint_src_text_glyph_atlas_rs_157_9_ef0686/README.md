# epaint/src/text/glyph_atlas.rs:157:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/glyph_atlas.rs:157:9`
- last API call: `epaint::Fonts::layout_uncached`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2453233908765529841

## Message

```
Bad pixels_per_point 0
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
use egui::Style;
use egui::TextStyle;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::TextFormat;

fn main() {
    let mut fonts = Fonts::new(TextOptions { max_texture_side: 10000000000000000usize, color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT, font_hinting: false, subpixel_binning: false }, FontDefinitions::default());
    let _ = fonts.layout_uncached(0.0f32, std::sync::Arc::new(LayoutJob::single_section("a".to_string(), TextFormat::simple(TextStyle::Heading.resolve(&Style::default()), Color32::LIGHT_RED))));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_glyph_atlas_rs_157_9_ef0686`).

## Output

```
thread 'main' (154046) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:157:9:
Bad pixels_per_point 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::text::glyph_atlas::OutlineGlyphKey>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:157:9
   3: <epaint::text::glyph_atlas::GlyphAtlas>::allocate_outline
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:297:19
   4: <epaint::text::fonts::FontsImpl>::allocate_glyph
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:753:14
   5: epaint::text::text_layout::layout_shaped_run
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:367:23
   6: epaint::text::text_layout::layout_section
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:603:13
   7: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:116:28
   8: <epaint::text::fonts::Fonts>::layout_uncached_for_viewport
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:291:9
   9: <epaint::text::fonts::Fonts>::layout_uncached
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:277:14
  10: repro_epaint_src_text_glyph_atlas_rs_157_9_ef0686::main
             at ./src/main.rs:13:19
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
