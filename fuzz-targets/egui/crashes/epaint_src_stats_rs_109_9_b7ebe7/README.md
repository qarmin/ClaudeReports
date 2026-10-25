# epaint/src/stats.rs:109:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/stats.rs:109:9`
- last API call: `epaint::stats::AllocInfo::num_elements`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17846831679824381292

## Message

```
Heterogenous element size
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_extras = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Style;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::stats::AllocInfo;
use epaint::text::FontDefinitions;

fn main() {
    let alloc_info = AllocInfo::from_galley(&Fonts::new(TextOptions { max_texture_side: 1000000000usize, color_transfer_function: FontColorTransferFunction::default(), font_hinting: false, subpixel_binning: false }, FontDefinitions::empty()).layout_uncached(1.0f32, std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Style::default(), &CodeTheme::from_style(&Style::default()), "", ""))));
    let _ = alloc_info.num_elements();
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_stats_rs_109_9_b7ebe7`).

## Output

```
thread 'main' (152697) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/stats.rs:109:9:
Heterogenous element size
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::stats::AllocInfo>::num_elements
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/stats.rs:109:9
   3: repro_epaint_src_stats_rs_109_9_b7ebe7::main
             at ./src/main.rs:12:24
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
