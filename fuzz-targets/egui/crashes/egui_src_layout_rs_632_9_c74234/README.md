# egui/src/layout.rs:632:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:632:9`
- last API call: `egui::Ui::set_height_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15163274959845984005

## Message

```
frame_rect is NaN: [[NaN NaN] - [NaN NaN]]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_extras = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Id;
use egui::Layout;
use egui::Ui;
use egui::UiBuilder;
use egui_extras::Size;
use emath::Align;

fn main() {
    let context = Context::default();
    let mut ui = Ui::new(context.clone(), Id::NULL, UiBuilder::default().layout(Layout::left_to_right(Align::Max)));
    ui.set_height_range(Size::relative(1.0f32).range());
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_632_9_c74234`).

## Output

```
thread 'main' (535856) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:632:9:
frame_rect is NaN: [[NaN NaN] - [NaN NaN]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Layout>::next_frame_ignore_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:632:9
   3: <egui::layout::Layout>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:660:26
   4: <egui::placer::Placer>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:228:14
   5: <egui::placer::Placer>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:249:25
   6: <egui::ui::Ui>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:751:21
   7: <egui::ui::Ui>::set_height_range::<emath::range::Rangef>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:838:14
   8: repro_egui_src_layout_rs_632_9_c74234::main
             at ./src/main.rs:12:8
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
