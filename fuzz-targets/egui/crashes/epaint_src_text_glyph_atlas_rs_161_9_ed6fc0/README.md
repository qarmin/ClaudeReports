# epaint/src/text/glyph_atlas.rs:161:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/glyph_atlas.rs:161:9`
- last API call: `epaint::Fonts::layout_uncached_for_viewport`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3507622105982489737

## Message

```
Bad px_scale_factor: 0
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
    let _ = Fonts::new(TextOptions::default(), FontDefinitions::default()).layout_uncached_for_viewport(1.0f32, ViewportKey::default(), std::sync::Arc::new(LayoutJob::simple_singleline("a".to_string(), FontId::monospace(0.0f32), Color32::ORANGE)));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_glyph_atlas_rs_161_9_ed6fc0`).

## Output

```
thread 'main' (147428) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:161:9:
Bad px_scale_factor: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::text::glyph_atlas::OutlineGlyphKey>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:161:9
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
   9: repro_epaint_src_text_glyph_atlas_rs_161_9_ed6fc0::main
             at ./src/main.rs:10:76
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
