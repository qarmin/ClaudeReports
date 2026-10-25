# out of memory in krilla::text::Font::new

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: out of memory
- last API call: `krilla::text::Font::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14643501289030435627

## Message

```
process ended abnormally (out of memory) while calling krilla::text::Font::new
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
use krilla::action::ResetFormAction;
use krilla::annotation::AnnotationType;
use krilla::annotation::LinkAnnotation;
use krilla::annotation::LinkBorder;
use krilla::annotation::Target;
use krilla::color::rgb::Color;
use krilla::configure::Accessibility;
use krilla::configure::ConfigurationBuilder;
use krilla::destination::Destination;
use krilla::destination::XyzDestination;
use krilla::form::kind::Text;
use krilla::geom::PathBuilder;
use krilla::geom::Point;
use krilla::geom::Quadrilateral;
use krilla::geom::Rect;
use krilla::image::Image;
use krilla::paint::LineJoin;
use krilla::tagging::GlyphOrientationVertical;
use krilla::tagging::kind::Lbl;
use krilla::tagging::kind::THead;
use krilla::text::Font;

fn main() {
    let _ = Image::from_webp(Data::from(std::sync::Arc::new(include_bytes!("mutated_0_rgba8.webp").to_vec())), true);
    let _ = ResetFormAction::Include(vec!["مرحبا".to_string()]);
    let mut path_builder = PathBuilder::default();
    path_builder.quad_to(-50.2f32, 0.0f32, 28.6f32, -1.6f32);
    let _ = <krilla::icc::ICCProfile<4>>::new(&include_bytes!("sRGB-v4.icc").to_vec());
    let _ = GlyphOrientationVertical::Clockwise90;
    let _ = Quadrilateral([Point { x: -0.0016725775f32, y: -2.2978705e-26f32 }, Point::default(), Point::from_xy(1.0f32, 256.0f32), Point::default()]);
    let _ = Lbl;
    let destination = Destination::from(XyzDestination::new(usize::MAX, Point { x: 82.2f32, y: f32::MAX }));
    let _ = AnnotationType::Link(LinkAnnotation::new(Rect::from_xywh(0.0f32, -0.0f32, 0.5f32, 0.5f32).unwrap(), Target::Destination(destination.clone())).with_border(LinkBorder::new(f32::MAX, krilla::color::Color::from(Color::default()))));
    let _ = Font::new(Data::from(include_bytes!("mutated_2_LatinModernRoman-Regular.otf").to_vec()), 3u32);
    let accessibility = Accessibility::from(Accessibility::UA1);
    let _ = accessibility.min();
    let _ = ConfigurationBuilder::default();
    let _ = THead;
    let _ = Text::default();
    let _ = XyzDestination::new(1usize, Point::from_xy(f32::MIN, 65536.0f32));
    let _ = XyzDestination::new(1usize, Point::default());
    let _ = LineJoin::Round;
    let _ = LineJoin::default();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_krilla_text_font_new_e160c4`).

## Output

```
memory allocation of 3722604800 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: krilla::graphics::image::decode_webp
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:797:27
  10: <krilla::graphics::image::Image>::from_webp::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:313:42
  11: <krilla::util::deferred::Deferred<core::result::Result<krilla::graphics::image::Repr, alloc::string::String>>>::new::<<krilla::graphics::image::Image>::from_webp::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/util.rs:188:18
  12: <krilla::graphics::image::Image>::from_webp
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:313:20
  13: repro_out_of_memory_in_krilla_text_font_new_e160c4::main
             at ./src/main.rs:25:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
