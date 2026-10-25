# krilla/src/graphics/shading_function.rs:114:60

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/graphics/shading_function.rs:114:60`
- last API call: `krilla::surface::Surface::draw_text`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
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
use krilla::document::Document;
use krilla::geom::Point;
use krilla::geom::Transform;
use krilla::num::NormalizedF32;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let font = Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap();
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    let fill = Fill { paint: Paint::from(LinearGradient { x1: 0.0f32, y1: f32::INFINITY, x2: 0.0f32, y2: 0.0f32, transform: Transform::from_rotate(0.0f32), spread_method: SpreadMethod::Reflect, stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::default() };
    surface.set_fill(Some(fill.clone()));
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), font.clone(), 0.0f32, "a", false, TextDirection::Auto);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_graphics_shading_function_rs_114_60_e6c201`).

## Output

```
thread 'main' (251712) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:114:60:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<krilla::geom::Transform>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: krilla::graphics::shading_function::get_expanded_bbox
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:114:60
   6: <krilla::graphics::paint::LinearGradient as krilla::graphics::shading_function::GradientPropertiesExt>::gradient_properties
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:159:29
   7: <krilla::content::ContentBuilder>::content_set_fill_stroke_properties::<<krilla::content::ContentBuilder>::content_set_fill_properties::set_pattern_fn, krilla::content::set_solid_fill>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1133:62
   8: <krilla::content::ContentBuilder>::content_set_fill_properties
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1178:14
   9: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:438:16
  10: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#6}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:510:25
  11: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:712:17
  12: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
  13: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
  14: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:503:22
  15: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:358:39
  16: <krilla::surface::Surface>::draw_text
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:414:14
  17: repro_krilla_src_graphics_shading_function_rs_114_60_e6c201::main
             at ./src/main.rs:21:13
  18: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
