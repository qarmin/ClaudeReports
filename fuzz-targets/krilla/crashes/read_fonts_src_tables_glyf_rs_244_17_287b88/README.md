# read-fonts/src/tables/glyf.rs:244:17

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `read-fonts/src/tables/glyf.rs:244:17`
- source: https://docs.rs/crate/read-fonts/0.39.2/source/src/tables/glyf.rs#244
- last API call: `krilla::surface::Surface::draw_text`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::page::PageSettings;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page_with(PageSettings::from_wh(1.0f32, 1.0f32).unwrap());
    let mut surface = page.surface();
    surface.draw_text(Point { x: 0.0f32, y: 0.0f32 }, Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap(), 0.0f32, "P", true, TextDirection::BottomToTop);
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_tables_glyf_rs_244_17_287b88`).

## Output

```
thread 'main' (251305) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.41.0/src/tables/glyf.rs:244:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <read_fonts::tables::glyf::SimpleGlyph>::read_points_fast::<i32>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.41.0/src/tables/glyf.rs:244:17
   4: <skrifa::outline::glyf::FreeTypeScaler as skrifa::outline::glyf::Scaler>::load_simple
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/glyf/mod.rs:614:15
   5: <skrifa::outline::glyf::FreeTypeScaler as skrifa::outline::glyf::Scaler>::load
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/glyf/mod.rs:301:49
   6: <skrifa::outline::glyf::FreeTypeScaler>::scale
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/glyf/mod.rs:504:14
   7: <skrifa::outline::OutlineGlyph>::draw_unhinted::<skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder>::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/mod.rs:420:38
   8: skrifa::outline::memory::with_temporary_memory::stack_mem::<512, core::result::Result<skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError>, <skrifa::outline::OutlineGlyph>::draw_unhinted<skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/memory.rs:10:9
   9: skrifa::outline::memory::with_temporary_memory::<core::result::Result<skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError>, <skrifa::outline::OutlineGlyph>::draw_unhinted<skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/memory.rs:15:9
  10: skrifa::outline::with_temporary_memory::<core::result::Result<skrifa::outline::AdjustedMetrics, skrifa::outline::error::DrawError>, <skrifa::outline::OutlineGlyph>::draw_unhinted<skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/mod.rs:769:13
  11: <skrifa::outline::OutlineGlyph>::draw_unhinted::<skrifa::instance::LocationRef, krilla::text::glyph::outline::OutlineBuilder>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/mod.rs:415:17
  12: <skrifa::outline::OutlineGlyph>::draw::<skrifa::outline::DrawSettings, krilla::text::glyph::outline::OutlineBuilder>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/skrifa-0.44.0/src/outline/mod.rs:361:22
  13: krilla::text::glyph::outline::glyph_path
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/outline.rs:15:14
  14: krilla::text::glyph::outline::draw_glyph
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/outline.rs:32:21
  15: krilla::text::glyph::draw_glyph::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/mod.rs:85:21
  16: <core::option::Option<()>>::or_else::<krilla::text::glyph::draw_glyph::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1652:21
  17: krilla::text::glyph::draw_glyph
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/mod.rs:85:10
  18: <krilla::surface::Surface>::outline_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:278:13
  19: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:307:18
  20: <krilla::surface::Surface>::draw_text
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:414:14
  21: repro_read_fonts_src_tables_glyf_rs_244_17_287b88::main
             at ./src/main.rs:13:13
  22: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
