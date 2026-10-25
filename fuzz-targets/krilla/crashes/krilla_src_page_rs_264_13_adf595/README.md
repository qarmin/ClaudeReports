# krilla/src/page.rs:264:13

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/page.rs:264:13`
- last API call: `krilla::page::Page::add_widget_annotation[u32]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Called add_widget_annotation with a non-widget annotation
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::action::Action;
use krilla::action::ResetFormAction;
use krilla::annotation::Annotation;
use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;
use krilla::document::Document;
use krilla::geom::Rect;

fn main() {
    let rect = Rect::from(Rect::from_xywh(0.0f32, 0.0f32, 0.0f32, 0.0f32).unwrap());
    let action = Action::from(ResetFormAction::All);
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut form_field = <krilla::form::FormField<u32> as ::core::default::Default>::default();
    let _ = page.add_widget_annotation::<u32>(&mut form_field, Annotation::from(LinkAnnotation::new(rect, Target::Action(action.clone()))));
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_page_rs_264_13_adf595`).

## Output

```
thread 'main' (250817) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/page.rs:264:13:
Called add_widget_annotation with a non-widget annotation
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <krilla::page::Page>::add_widget_annotation::<u32>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/page.rs:264:13
   3: repro_krilla_src_page_rs_264_13_adf595::main
             at ./src/main.rs:15:18
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
