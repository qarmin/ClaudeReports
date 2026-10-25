# krilla/src/text/group.rs:115:29

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/text/group.rs:115:29`
- last API call: `krilla::surface::Surface::draw_glyphs`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 1 is out of bounds for string of length 0
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
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.draw_glyphs(Point::from_xy(0.0f32, 0.0f32), &[KrillaGlyph { glyph_id: GlyphId::new(1u32), text_range: 0usize..1usize, x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32, location: None }, KrillaGlyph { glyph_id: GlyphId::new(0u32), text_range: 0usize..0usize, x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32, location: None }], Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), "", 0.0f32, false);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_text_group_rs_115_29_dda73d`).

## Output

```
thread 'main' (250859) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/group.rs:115:29:
end byte index 1 is out of bounds for string of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <krilla::text::group::GlyphSpanner<_> as core::iter::traits::iterator::Iterator>::next::func::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/group.rs:115:29
   8: <krilla::text::group::GlyphSpanner<krilla::text::glyph::KrillaGlyph> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/group.rs:152:49
   9: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:738:37
  10: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
  11: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
  12: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:503:22
  13: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:373:39
  14: repro_krilla_src_text_group_rs_115_29_dda73d::main
             at ./src/main.rs:13:13
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
