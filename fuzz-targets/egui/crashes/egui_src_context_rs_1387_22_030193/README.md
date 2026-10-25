# egui/src/context.rs:1387:22

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/context.rs:1387:22`
- last API call: `egui::Ui::add[Hyperlink]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16833604619655123893

## Message

```
No fonts available until first call to Context::run()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Hyperlink;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::unique(0u8), UiBuilder::default());
    let _ = ui.add(Hyperlink::new("".to_string()));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_context_rs_1387_22_030193`).

## Output

```
thread 'main' (75190) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1387:22:
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
   5: <egui::context::Context>::fonts_mut::<alloc::sync::Arc<epaint::text::text_layout_types::Galley>, <egui::widgets::label::Label>::layout_in_ui::{closure#6}>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1387:22
   6: <egui::context::Context>::write::<alloc::sync::Arc<epaint::text::text_layout_types::Galley>, <egui::context::Context>::fonts_mut<alloc::sync::Arc<epaint::text::text_layout_types::Galley>, <egui::widgets::label::Label>::layout_in_ui::{closure#6}>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:920:9
   7: <egui::context::Context>::fonts_mut::<alloc::sync::Arc<epaint::text::text_layout_types::Galley>, <egui::widgets::label::Label>::layout_in_ui::{closure#6}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1380:14
   8: <egui::widgets::label::Label>::layout_in_ui
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/label.rs:257:29
   9: <egui::widgets::hyperlink::Link as egui::widgets::Widget>::ui
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/hyperlink.rs:42:52
  10: <egui::ui::Ui>::add::<egui::widgets::hyperlink::Link>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1647:16
  11: <egui::widgets::hyperlink::Hyperlink as egui::widgets::Widget>::ui
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/hyperlink.rs:129:27
  12: <egui::ui::Ui>::add::<egui::widgets::hyperlink::Hyperlink>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1647:16
  13: repro_egui_src_context_rs_1387_22_030193::main
             at ./src/main.rs:9:16
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
