# egui/src/grid.rs:107:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/grid.rs:107:9`
- last API call: `egui::style::Interaction::ui`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 739605285782355302

## Message

```
Grid not yet available for right-to-left layouts
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
use egui::Ui;
use egui::UiBuilder;
use egui::ViewportId;
use egui::style::Interaction;
use emath::Rect;

fn main() {
    let mut interaction = Interaction::default();
    let mut ui = Ui::new(Context::default(), ViewportId::from_hash_of(0u8).root_ui_id(), UiBuilder::default().max_rect(Rect::EVERYTHING));
    let mut ui2 = ui.new_child(UiBuilder::new());
    interaction.ui(&mut ui2);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_grid_rs_107_9_50db67`).

## Output

```
thread 'main' (330030) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:107:9:
Grid not yet available for right-to-left layouts
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::grid::GridLayout>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:107:9
   3: <egui::grid::Grid>::show_dyn::<()>::{closure#3}::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:485:23
   4: <<egui::grid::Grid>::show_dyn<()>::{closure#3}::{closure#0} as core::ops::function::FnOnce<(&mut egui::ui::Ui,)>>::call_once::{shim:vtable#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   5: <alloc::boxed::Box<dyn for<'a> core::ops::function::FnOnce<(&'a mut egui::ui::Ui,), Output = ()>> as core::ops::function::FnOnce<(&mut egui::ui::Ui,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2320:9
   6: <egui::ui::Ui>::scope_dyn::<()>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2336:19
   7: <egui::ui::Ui>::allocate_ui_with_layout_dyn::<()>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1469:14
   8: <egui::ui::Ui>::horizontal_with_main_wrap_dyn::<()>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2513:14
   9: <egui::ui::Ui>::horizontal::<(), <egui::grid::Grid>::show_dyn<()>::{closure#3}::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2441:14
  10: <egui::grid::Grid>::show_dyn::<()>::{closure#3}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:476:16
  11: <<egui::grid::Grid>::show_dyn<()>::{closure#3} as core::ops::function::FnOnce<(&mut egui::ui::Ui,)>>::call_once::{shim:vtable#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  12: <alloc::boxed::Box<dyn for<'a> core::ops::function::FnOnce<(&'a mut egui::ui::Ui,), Output = ()>> as core::ops::function::FnOnce<(&mut egui::ui::Ui,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2320:9
  13: <egui::ui::Ui>::scope_dyn::<()>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2336:19
  14: <egui::ui::Ui>::scope_builder::<(), <egui::grid::Grid>::show_dyn<()>::{closure#3}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2324:14
  15: <egui::grid::Grid>::show_dyn::<()>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:475:12
  16: <egui::grid::Grid>::show::<(), <egui::style::Interaction>::ui::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/grid.rs:428:14
  17: <egui::style::Interaction>::ui
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/style.rs:2207:14
  18: repro_egui_src_grid_rs_107_9_50db67::main
             at ./src/main.rs:12:17
  19: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
