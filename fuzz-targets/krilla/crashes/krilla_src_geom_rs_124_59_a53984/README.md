# krilla/src/geom.rs:124:59

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/geom.rs:124:59`
- last API call: `krilla::surface::Surface::draw_image`
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
use krilla::document::Document;
use krilla::geom::Rect;
use krilla::geom::Size;
use krilla::image::Image;
use krilla::page::PageSettings;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page_with(PageSettings::from_wh(1.0f32, 1.0f32).unwrap());
    let mut surface = page.surface();
    let mut stream_builder = surface.stream_builder();
    let mut surface2 = stream_builder.surface_with_bbox(Rect::from_xywh(0.0f32, f32::MAX, 0.0f32, 0.0f32).unwrap());
    surface2.draw_image(Image::from_rgba8(vec![], 0u32, 0u32), Size::from_wh(1.0f32, 1.0f32).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_geom_rs_124_59_a53984`).

## Output

```
thread 'main' (251200) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/geom.rs:124:59:
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
   4: <core::option::Option<krilla::geom::Rect>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <krilla::geom::Rect>::expand
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/geom.rs:124:59
   6: <krilla::content::ContentBuilder>::expand_bbox
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:207:18
   7: <krilla::content::ContentBuilder>::draw_image::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:972:20
   8: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::draw_image::{closure#0}, <krilla::content::ContentBuilder>::draw_image::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1029:9
   9: <krilla::content::ContentBuilder>::draw_image
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:966:14
  10: <krilla::surface::Surface>::draw_image
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:553:14
  11: repro_krilla_src_geom_rs_124_59_a53984::main
             at ./src/main.rs:13:14
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
