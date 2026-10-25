# krilla/src/graphics/shading_function.rs:254:54

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/graphics/shading_function.rs:254:54`
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
use krilla::geom::Size;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::SweepGradient;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    let spread_method = SpreadMethod::Pad;
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32, 1.0f32).unwrap()));
    let mut surface = page.surface();
    let transform = surface.cur_transform();
    surface.set_fill(Some(Fill { paint: Paint::from(SweepGradient { cx: 0.0f32, cy: 1.0f32, start_angle: 0.0f32, end_angle: 1.0f32, transform: transform, spread_method: spread_method, stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::default() }));
    surface.draw_text(Point::default(), Font::new(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32).unwrap(), 0.0f32, "Z", false, TextDirection::LeftToRight);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_graphics_shading_function_rs_254_54_a1b1fd`).

## Output

```
thread 'main' (251398) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:254:54:
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
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/shading_function.rs:254:54
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
  15: <krilla::content::ContentBuilder>::content_set_fill_stroke_properties::<<krilla::content::ContentBuilder>::content_set_fill_properties::set_pattern_fn, krilla::content::set_solid_fill>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1102:32
  16: <krilla::content::ContentBuilder>::content_set_fill_stroke_properties::<<krilla::content::ContentBuilder>::content_set_fill_properties::set_pattern_fn, krilla::content::set_solid_fill>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1142:17
  17: <krilla::content::ContentBuilder>::content_set_fill_properties
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1178:14
  18: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:438:16
  19: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>::{closure#6}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:510:25
  20: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:712:17
  21: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#0}, <krilla::content::ContentBuilder>::fill_stroke_glyph_run<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1046:9
  22: <krilla::content::ContentBuilder>::fill_stroke_glyph_run::<<krilla::content::ContentBuilder>::draw_glyphs<krilla::text::glyph::KrillaGlyph>::{closure#6}, krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:699:14
  23: <krilla::content::ContentBuilder>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:503:22
  24: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:358:39
  25: <krilla::surface::Surface>::draw_text
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:414:14
  26: repro_krilla_src_graphics_shading_function_rs_254_54_a1b1fd::main
             at ./src/main.rs:23:13
  27: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
