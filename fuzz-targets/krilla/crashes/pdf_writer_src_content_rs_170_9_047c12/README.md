# pdf-writer/src/content.rs:170:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `pdf-writer/src/content.rs:170:9`
- last API call: `krilla::surface::Surface::draw_glyphs`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
line width must be positive
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
use krilla::color::cmyk::Color;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::Paint;
use krilla::paint::Stroke;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.set_stroke(Some(Stroke { paint: Paint::from(Color::new(0u8, 0u8, 0u8, 0u8)), width: -68.0f32, miter_limit: 0.0f32, line_cap: LineCap::default(), line_join: LineJoin::default(), opacity: NormalizedF32::ZERO, dash: None }));
    surface.draw_glyphs(Point::default(), &[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..0usize, None)], Font::new_variable(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32, &[]).unwrap(), "", 0.0f32, false);
}
```

Run with `cargo run` in this directory (package `repro_pdf_writer_src_content_rs_170_9_047c12`).

## Output

```
thread 'main' (251163) panicked at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/content.rs:170:9:
line width must be positive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <pdf_writer::content::Content>::set_line_width
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/content.rs:170:9
   3: <krilla::content::ContentBuilder>::content_set_stroke_properties
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1239:26
   4: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#2}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:454:16
   5: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#7}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:529:25
   6: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:712:17
   7: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
   8: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
   9: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:522:22
  10: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:344:39
  11: repro_pdf_writer_src_content_rs_170_9_047c12::main
             at ./src/main.rs:19:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
