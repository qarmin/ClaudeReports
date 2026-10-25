# egui/src/context.rs:1365:22

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/context.rs:1365:22`
- last API call: `egui::introspection::font_id_ui`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8154987506540507667

## Message

```
No fonts available until first call to Context::run()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_kittest = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Ui;
use egui::UiBuilder;
use epaint::FontFamily;
use epaint::FontId;

fn main() {
    let context = Context::default();
    let harness = <egui_kittest::Harness<u8>>::new_ui_state(|_, _| (), 0u8);
    let mut ui = Ui::new(context.clone(), harness.ui_id(), UiBuilder::new());
    let mut ui2 = ui.new_child(UiBuilder::new());
    let mut font_id = FontId::new(0.0f32, FontFamily::Monospace);
    egui::introspection::font_id_ui(&mut ui2, &mut font_id);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_context_rs_1365_22_f4c9b0`).

## Output

```
thread 'main' (151914) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1365:22:
No fonts available until first call to Context::run()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<&mut epaint::text::fonts::Fonts>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <egui::context::Context>::fonts::<alloc::vec::Vec<epaint::text::font_id::FontFamily>, egui::introspection::font_id_ui::{closure#0}>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1365:22
   6: <egui::context::Context>::write::<alloc::vec::Vec<epaint::text::font_id::FontFamily>, <egui::context::Context>::fonts<alloc::vec::Vec<epaint::text::font_id::FontFamily>, egui::introspection::font_id_ui::{closure#0}>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:920:9
   7: <egui::context::Context>::fonts::<alloc::vec::Vec<epaint::text::font_id::FontFamily>, egui::introspection::font_id_ui::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1359:14
   8: egui::introspection::font_id_ui
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/introspection.rs:18:23
   9: repro_egui_src_context_rs_1365_22_f4c9b0::main
             at ./src/main.rs:13:5
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
