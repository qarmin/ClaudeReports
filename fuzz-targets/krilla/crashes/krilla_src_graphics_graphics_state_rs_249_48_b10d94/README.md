# krilla/src/graphics/graphics_state.rs:249:48

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/graphics/graphics_state.rs:249:48`
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
use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Size;
use krilla::image::Image;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.draw_image(Image::from_png(Data::from(include_bytes!("data_0.bin").to_vec()), false).unwrap(), Size::from_wh(f32::MAX, 1.0f32).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_graphics_graphics_state_rs_249_48_b10d94`).

## Output

```
thread 'main' (250764) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/graphics_state.rs:249:48:
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
   5: <krilla::graphics::graphics_state::GraphicsStates>::transform_bbox
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/graphics_state.rs:249:48
   6: <krilla::content::ContentBuilder>::expand_bbox
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:205:45
   7: <krilla::content::ContentBuilder>::draw_image::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:972:20
   8: <krilla::content::ContentBuilder>::apply_isolated_op::<<krilla::content::ContentBuilder>::draw_image::{closure#0}, <krilla::content::ContentBuilder>::draw_image::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1029:9
   9: <krilla::content::ContentBuilder>::draw_image
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:966:14
  10: <krilla::surface::Surface>::draw_image
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:553:14
  11: repro_krilla_src_graphics_graphics_state_rs_249_48_b10d94::main
             at ./src/main.rs:11:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
