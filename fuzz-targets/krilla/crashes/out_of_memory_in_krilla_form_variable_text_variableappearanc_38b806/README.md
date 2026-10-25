# out of memory in krilla::form::variable_text::VariableAppearance {literal}

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: out of memory
- last API call: `krilla::form::variable_text::VariableAppearance {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15043326452143017984

## Message

```
process ended abnormally (out of memory) while calling krilla::form::variable_text::VariableAppearance {literal}
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
use krilla::action::Action;
use krilla::action::ResetFormAction;
use krilla::color::rgb::Color;
use krilla::configure::Accessibility;
use krilla::configure::ValidationError;
use krilla::configure::Validators;
use krilla::configure::validate::VersionedFeature;
use krilla::destination::XyzDestination;
use krilla::embed::AssociationKind;
use krilla::form::variable_text::FormFont;
use krilla::form::variable_text::VariableAppearance;
use krilla::geom::Point;
use krilla::geom::Rect;
use krilla::geom::Transform;
use krilla::image::Image;
use krilla::image::ImageColorspace;
use krilla::num::NormalizedF32;
use krilla::outline::OutlineNode;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::Stroke;
use krilla::paint::StrokeDash;
use krilla::tagging::kind::Datetime;
use krilla::text::StandardFont;

fn main() {
    let transform = Transform::default();
    let f32_v = transform.tx();
    let _ = VariableAppearance { font: FormFont::Standard(StandardFont::TimesRoman), font_size: -1e30f32, paint: Some(Paint::from(Color::new(255u8, 255u8, 19u8))), opacity: NormalizedF32::ZERO };
    let _ = Accessibility::UA1;
    let _ = AssociationKind::Data;
    let _ = Image::from_webp(Data::from(include_bytes!("mutated_0_rgba8.webp").to_vec()), true);
    let form_field = <krilla::form::FormField<krilla::form::kind::PushButton> as ::core::default::Default>::default();
    let _ = form_field.with_location(None);
    let outline_node = OutlineNode::new("i_Q&_\u{ce100}ZU-pv".to_string(), XyzDestination::from(XyzDestination::from(XyzDestination::new(0usize, Point::default()))));
    let mut outline_node2 = outline_node.clone().with_open(true);
    outline_node2.push_child(outline_node.clone());
    let _ = Rect::from_ltrb(f32::NAN, 655.0f32, -1.1754944e-38f32, 1000.0f32);
    let _ = ImageColorspace::Rgb;
    let _ = Action::from(ResetFormAction::from(ResetFormAction::All));
    let validators = Validators::default();
    let validators2 = validators.prohibits(&ValidationError::EmbeddedPDF(Some(std::num::NonZero::new(27u64).unwrap()))).unwrap();
    let _ = validators2.prohibits(&ValidationError::ContainsPostScript(Some(std::num::NonZero::new(62u64).unwrap())));
    let versioned_feature = VersionedFeature::StructureOrderTabbing;
    let _ = versioned_feature.minimum_pdf_version();
    let _ = Stroke::default();
    let _ = LinearGradient { x1: 1.1920929e-7f32, y1: 81.9f32, x2: 0.5f32, y2: -0.0f32, transform: transform, spread_method: SpreadMethod::Repeat, stops: vec![], anti_alias: true };
    let _ = Paint::from(krilla::color::luma::Color::default());
    let _ = VariableAppearance { font: FormFont::Standard(StandardFont::Courier), font_size: f32::MAX, paint: None, opacity: NormalizedF32::new(f32::MIN).unwrap() };
    let _ = ResetFormAction::Include(vec!["\0".to_string(), "ZK(0\u{575cf}5 6)]F😀o&".to_string(), "Zażółć gęślą jaźń".to_string()]);
    let _ = Datetime;
    let _ = Stroke::default();
    let _ = StrokeDash { array: vec![-15.7f32, 445.0f32, f32_v, f32::NAN, 1.1754944e-38f32], offset: 10000000000.0f32 };
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_krilla_form_variable_text_variableappearanc_38b806`).

## Output

```
memory allocation of 4980896000 bytes failed
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
  13: repro_out_of_memory_in_krilla_form_variable_text_variableappearanc_38b806::main
             at ./src/main.rs:34:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
