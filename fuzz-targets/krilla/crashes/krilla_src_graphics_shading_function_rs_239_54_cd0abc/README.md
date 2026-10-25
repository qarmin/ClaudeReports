# krilla/src/graphics/shading_function.rs:239:54

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/graphics/shading_function.rs:239:54`
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
use krilla::document::Document;
use krilla::geom::Point;
use krilla::geom::Transform;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::Stroke;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32, y2: 0.0f32, transform: Transform::identity(), spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false });
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.set_stroke(Some(Stroke { paint: paint.clone(), width: 1.0f32, miter_limit: 0.0f32, line_cap: LineCap::Square, line_join: LineJoin::Bevel, opacity: NormalizedF32::new(0.0f32).unwrap(), dash: None }));
    surface.draw_text(Point::from_xy(0.0f32, 0.0f32), Font::new(Data::from(include_bytes!("data_0.bin").to_vec()), 0u32).unwrap(), 0.0f32, "1", false, TextDirection::BottomToTop);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_graphics_shading_function_rs_239_54_cd0abc`).

## Output

```
thread 'main' (251613) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:239:54:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[krilla::graphics::paint::Stop]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[krilla::graphics::paint::Stop] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<krilla::graphics::paint::Stop> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <krilla::graphics::shading_function::ShadingFunction as krilla::serialize::Cacheable>::serialize
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:239:54
   7: <krilla::serialize::SerializeContext>::register_cacheable::<krilla::graphics::shading_function::ShadingFunction>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:620:20
   8: <krilla::serialize::SerializeContext>::register_cached::<krilla::graphics::shading_function::ShadingFunction, <krilla::serialize::SerializeContext>::register_cacheable<krilla::graphics::shading_function::ShadingFunction>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:605:13
   9: <krilla::serialize::SerializeContext>::register_cacheable::<krilla::graphics::shading_function::ShadingFunction>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:619:14
  10: <krilla::graphics::shading_pattern::ShadingPattern as krilla::serialize::Cacheable>::serialize
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_pattern.rs:48:30
  11: <krilla::serialize::SerializeContext>::register_cacheable::<krilla::graphics::shading_pattern::ShadingPattern>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:620:20
  12: <krilla::serialize::SerializeContext>::register_cached::<krilla::graphics::shading_pattern::ShadingPattern, <krilla::serialize::SerializeContext>::register_cacheable<krilla::graphics::shading_pattern::ShadingPattern>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:605:13
  13: <krilla::serialize::SerializeContext>::register_cacheable::<krilla::graphics::shading_pattern::ShadingPattern>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:619:14
  14: <krilla::serialize::SerializeContext>::register_resourceable::<krilla::graphics::shading_pattern::ShadingPattern>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:632:28
  15: <krilla::content::ContentBuilder>::content_set_fill_stroke_properties::<<krilla::content::ContentBuilder>::content_set_stroke_properties::set_pattern_fn, <krilla::content::ContentBuilder>::content_set_stroke_properties::set_solid_fn>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1102:32
  16: <krilla::content::ContentBuilder>::content_set_fill_stroke_properties::<<krilla::content::ContentBuilder>::content_set_stroke_properties::set_pattern_fn, <krilla::content::ContentBuilder>::content_set_stroke_properties::set_solid_fn>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1134:17
  17: <krilla::content::ContentBuilder>::content_set_stroke_properties
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1226:14
  18: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#2}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:454:16
  19: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#7}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:529:25
  20: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:712:17
  21: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
  22: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#7}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
  23: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:522:22
  24: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:344:39
  25: <krilla::surface::Surface>::draw_text
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:414:14
  26: repro_krilla_src_graphics_shading_function_rs_239_54_cd0abc::main
             at ./src/main.rs:21:13
  27: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
