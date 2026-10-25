# krilla/src/surface.rs:644:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/surface.rs:644:9`
- last API call: `krilla::surface::Surface::push_opacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.push_instructions.is_empty()
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
use krilla::geom::Size;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;

fn main() {
    let mut document = Document::default();
    let mut page = document.start_page_with(PageSettings::new(Size::from_wh(1.0f32, 1.0f32).unwrap()));
    let mut surface = page.surface();
    surface.push_isolated();
    surface.pop();
    surface.push_opacity(NormalizedF32::ONE);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_surface_rs_644_9_e2e9a5`).

## Output

```
thread 'main' (251664) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9:
assertion failed: self.push_instructions.is_empty()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <krilla::surface::Surface as core::ops::drop::Drop>::drop
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9
   4: core::ptr::drop_glue::<krilla::surface::Surface>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   5: repro_krilla_src_surface_rs_644_9_e2e9a5::main
             at ./src/main.rs:13:1
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
