# egui/src/layout.rs:662:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:662:9`
- last API call: `egui::Ui::put[Hyperlink]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12894291316652827517

## Message

```
rect is NaN: [[NaN inf] - [NaN inf]]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
egui_kittest = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Hyperlink;
use egui::Ui;
use egui::UiBuilder;
use emath::Rect;

fn main() {
    let harness = <egui_kittest::Harness<u32>>::new_ui_state(|_, _| (), 0u32);
    let mut ui = Ui::new(Context::default(), harness.ui_id(), UiBuilder::new());
    let _ = ui.put(Rect::everything_below(1.0f32), Hyperlink::new("".to_string()));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_662_9_86d5cf`).

## Output

```
thread 'main' (84688) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:662:9:
rect is NaN: [[NaN inf] - [NaN inf]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Layout>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:662:9
   3: <egui::layout::Layout>::next_widget_position
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:669:14
   4: <egui::layout::Layout>::region_from_max_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:408:25
   5: <egui::placer::Placer>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:16:29
   6: <egui::ui::Ui>::new_child
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:267:22
   7: <egui::ui::Ui>::scope_dyn::<egui::response::Response>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2334:33
   8: <egui::ui::Ui>::scope_builder::<egui::response::Response, <egui::ui::Ui>::put<egui::widgets::hyperlink::Hyperlink>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:2324:14
   9: <egui::ui::Ui>::put::<egui::widgets::hyperlink::Hyperlink>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1692:14
  10: repro_egui_src_layout_rs_662_9_86d5cf::main
             at ./src/main.rs:10:16
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
