# pdf-writer/src/buf.rs:195:37

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `pdf-writer/src/buf.rs:195:37`
- last API call: `krilla::surface::Surface::draw_text`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9386572252662773744

## Message

```
attempt to negate with overflow
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
    let mut page = document.start_page_with(PageSettings::default());
    let mut surface = page.surface();
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), -2147483600.0f32, "a", false, TextDirection::Auto);
}
```

Run with `cargo run` in this directory (package `repro_pdf_writer_src_buf_rs_195_37_25e304`).

## Output

```
thread 'main' (1618324) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:429:5:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32>::abs
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/int_macros.rs:3635:17
   4: <pdf_writer::buf::Limits>::register_int
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/buf.rs:195:37
   5: <pdf_writer::buf::Buf>::push_int
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/buf.rs:46:21
   6: <pdf_writer::buf::Buf>::push_float
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/buf.rs:55:18
   7: <f32 as pdf_writer::object::sealed::Sealed>::write
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/object.rs:70:13
   8: <pdf_writer::object::Obj>::primitive::<f32>
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/object.rs:701:15
   9: <pdf_writer::content::Operation>::operand::<f32>
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/content.rs:102:20
  10: <pdf_writer::content::Content>::set_font
             at /home/rafal/.cargo/git/checkouts/pdf-writer-6ef6756ec5e515e5/fdf0901/src/content.rs:546:37
  11: <krilla::content::ContentBuilder>::encode_consecutive_glyph_run::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:563:22
  12: <krilla::content::ContentBuilder>::fill_stroke_glyph_group::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:857:14
  13: <krilla::content::ContentBuilder>::fill_stroke_glyph_span::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:806:18
  14: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:755:24
  15: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill... (+160 chars)
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
  16: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
  17: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:503:22
  18: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:373:39
  19: <krilla::surface::Surface>::draw_text
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:414:14
  20: repro_pdf_writer_src_buf_rs_195_37_25e304::main
             at ./src/main.rs:13:13
  21: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
