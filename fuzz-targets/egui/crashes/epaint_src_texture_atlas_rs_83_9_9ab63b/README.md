# epaint/src/texture_atlas.rs:83:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/texture_atlas.rs:83:9`
- last API call: `epaint::Fonts::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8827081474984789434

## Message

```
Tiny texture atlas
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let _ = Fonts::new(TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Gamma(0.0f32), font_hinting: false, subpixel_binning: false }, FontDefinitions::empty());
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_texture_atlas_rs_83_9_9ab63b`).

## Output

```
thread 'main' (13059) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/texture_atlas.rs:83:9:
Tiny texture atlas
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::texture_atlas::TextureAtlas>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/texture_atlas.rs:83:9
   3: <epaint::text::glyph_atlas::GlyphAtlas>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/glyph_atlas.rs:224:20
   4: <epaint::text::fonts::FontsImpl>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:514:21
   5: <epaint::text::fonts::Fonts>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:96:20
   6: repro_epaint_src_texture_atlas_rs_83_9_9ab63b::main
             at ./src/main.rs:7:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
