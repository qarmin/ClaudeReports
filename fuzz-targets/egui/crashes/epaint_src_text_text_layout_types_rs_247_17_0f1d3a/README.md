# epaint/src/text/text_layout_types.rs:247:17

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/text_layout_types.rs:247:17`
- last API call: `epaint::Fonts::layout_uncached`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16165544470711550734

## Message

```
LayoutJob has text but no sections: "a"
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use emath::Align;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::TextWrapping;

fn main() {
    let _ = Fonts::new(TextOptions { max_texture_side: 1000000000usize, color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT, font_hinting: false, subpixel_binning: false }, FontDefinitions::default()).layout_uncached(0.0f32, std::sync::Arc::new(LayoutJob { text: "a".to_string(), sections: vec![], wrap: TextWrapping { max_width: 0.0f32, max_rows: 0usize, break_anywhere: false, overflow_character: None }, first_row_min_height: 0.0f32, break_on_newline: false, halign: Align::Min, justify: false, round_output_to_gui: false, keep_trailing_whitespace: false }));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_text_layout_types_rs_247_17_0f1d3a`).

## Output

```
thread 'main' (85069) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:247:17:
LayoutJob has text but no sections: "a"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::text::text_layout_types::LayoutJob>::debug_sanity_check
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:247:17
   3: epaint::text::text_layout::layout
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout.rs:93:9
   4: <epaint::text::fonts::Fonts>::layout_uncached_for_viewport
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:291:9
   5: <epaint::text::fonts::Fonts>::layout_uncached
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/fonts.rs:277:14
   6: repro_epaint_src_text_text_layout_types_rs_247_17_0f1d3a::main
             at ./src/main.rs:10:220
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
