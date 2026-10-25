# egui/src/layout.rs:375:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:375:9`
- last API call: `egui::Layout::align_size_within_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15043172451607859591

## Message

```
Negative size: [-inf -inf]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Layout;
use emath::Align2;
use emath::Rect;
use emath::RectAlign;

fn main() {
    let vec2 = <emath::Vec2 as emath::GuiRounding>::round_to_pixel_center(RectAlign::TOP_START.child().to_sign(), 0.0f32);
    let _ = Layout::top_down_justified(Align2::CENTER_CENTER.y()).align_size_within_rect(vec2, Rect::ZERO);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_375_9_d15d97`).

## Output

```
thread 'main' (197819) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:375:9:
Negative size: [-inf -inf]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Layout>::align_size_within_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:375:9
   3: repro_egui_src_layout_rs_375_9_d15d97::main
             at ./src/main.rs:8:67
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
