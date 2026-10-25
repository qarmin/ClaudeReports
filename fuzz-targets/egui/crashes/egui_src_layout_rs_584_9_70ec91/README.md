# egui/src/layout.rs:584:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:584:9`
- last API call: `egui::Ui::set_max_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7933894138586046023

## Message

```
Negative size: [0.0 -340282346638528859811704183484516925440.0]
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
use egui::Ui;
use egui::UiBuilder;
use emath::Vec2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::NULL, UiBuilder::new());
    ui.set_max_size(Vec2::from([0.0f32, f32::MIN]));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_584_9_70ec91`).

## Output

```
thread 'main' (75703) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:584:9:
Negative size: [0.0 -340282346638528859811704183484516925440.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Layout>::next_frame_ignore_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:584:9
   3: <egui::layout::Layout>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:660:26
   4: <egui::placer::Placer>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:228:14
   5: <egui::placer::Placer>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:249:25
   6: <egui::ui::Ui>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:751:21
   7: <egui::ui::Ui>::set_max_size
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:739:14
   8: repro_egui_src_layout_rs_584_9_70ec91::main
             at ./src/main.rs:9:8
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
