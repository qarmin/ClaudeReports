# epaint/src/text/fonts.rs:806:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/fonts.rs:806:9`
- last API call: `epaint::FontsView::layout`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8374134675709336062

## Message

```
No glyph for "a" (U+0061) in Proportional. Installed fonts: []. The family has no fonts at all: install some with `FontDefinitions`, or enable the `default_fonts` feature to bundle egui's.
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
use ecolor::Rgba;
use egui::Context;
use egui::FontSelection;
use egui::Style;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::MissingGlyphPolicy;

fn main() {
    let rgba = Rgba::BLUE;
    let f32_v = rgba.intensity();
    let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let fonts2 = fonts.with_missing_glyph_policy(Context::default().missing_glyph_policy());
    let mut fonts3 = fonts2.with_missing_glyph_policy(MissingGlyphPolicy::Panic);
    let mut fonts_view = fonts3.with_pixels_per_point(f32_v);
    let color32 = Color32::LIGHT_BLUE;
    let _ = fonts_view.layout("a".to_string(), FontSelection::default().resolve(&Style::default()), color32, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_fonts_rs_806_9_a94fbe`).

## Output

```
thread 'main' (204458) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:806:9:
No glyph for "a" (U+0061) in Proportional. Installed fonts: []. The family has no fonts at all: install some with `FontDefinitions`, or enable the `default_fonts` feature to bundle egui's.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::text::fonts::FontsImpl>::on_missing_glyph
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:806:9
   3: <epaint::text::fonts::FontsImpl>::fontless_cluster
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:833:14
   4: epaint::text::text_layout::segment_into_runs
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:1538:32
   5: epaint::text::text_layout::layout_section
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:572:9
   6: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:116:28
   7: <epaint::text::galley_cache::GalleyCache>::layout_internal
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/galley_cache.rs:112:34
   8: <epaint::text::galley_cache::GalleyCache>::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/galley_cache.rs:134:14
   9: <epaint::text::fonts::FontsView>::layout_job
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:421:27
  10: <epaint::text::fonts::FontsView>::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:453:14
  11: repro_epaint_src_text_fonts_rs_806_9_a94fbe::main
             at ./src/main.rs:19:24
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
