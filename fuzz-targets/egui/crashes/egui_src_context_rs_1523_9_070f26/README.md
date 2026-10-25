# egui/src/context.rs:1523:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/context.rs:1523:9`
- last API call: `egui::Ui::interact_opt`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9479728271048598409

## Message

```
widget rect is NaN: [[NaN NaN] - [NaN NaN]]
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
use egui::Context;
use egui::Id;
use egui::InteractOptions;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Rect;

fn main() {
    let context = Context::default();
    let ui = Ui::new(context.clone(), Id::new(0u32), UiBuilder::default());
    let _ = ui.interact_opt(Rect::NAN, Context::default().viewport_id().root_ui_id(), Sense::CLICK, InteractOptions { move_to_top: false });
}
```

Run with `cargo run` in this directory (package `repro_egui_src_context_rs_1523_9_070f26`).

## Output

```
thread 'main' (227626) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1523:9:
widget rect is NaN: [[NaN NaN] - [NaN NaN]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::context::Context>::create_widget
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1523:9
   3: <egui::ui::Ui>::interact_opt
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:958:20
   4: repro_egui_src_context_rs_1523_9_070f26::main
             at ./src/main.rs:12:16
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
