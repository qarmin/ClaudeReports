# egui/src/layout.rs:490:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:490:9`
- last API call: `egui::Ui::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 11610960271314980742

## Message

```
avail is NaN: [[NaN inf] - [NaN inf]]
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
use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let harness = <egui_kittest::Harness<u32>>::new_ui_state(|_, _| (), 0u32);
    let _ = Ui::new(Context::default(), harness.ui_id(), UiBuilder::new().max_rect(Mesh::with_texture(TextureId::User(0u64)).calc_bounds()));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_490_9_0ef05b`).

## Output

```
thread 'main' (192071) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:490:9:
avail is NaN: [[NaN inf] - [NaN inf]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Layout>::available_from_cursor_max_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:490:9
   3: <egui::layout::Layout>::available_rect_before_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:414:14
   4: <egui::layout::Layout>::next_frame_ignore_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:589:35
   5: <egui::layout::Layout>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:660:26
   6: <egui::layout::Layout>::next_widget_position
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:669:14
   7: <egui::layout::Layout>::region_from_max_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:408:25
   8: <egui::placer::Placer>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:16:29
   9: <egui::ui::Ui>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:131:22
  10: repro_egui_src_layout_rs_490_9_0ef05b::main
             at ./src/main.rs:9:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
