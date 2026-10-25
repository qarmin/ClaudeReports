# egui/src/ui.rs:1390:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/ui.rs:1390:9`
- last API call: `egui::Ui::allocate_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7785917280780265565

## Message

```
rect is nan in advance_cursor_after_rect
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
use egui::PopupAnchor;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Pos2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::new(0u32), UiBuilder::default());
    let _ = ui.allocate_rect(PopupAnchor::from(Pos2::NAN).rect(egui::accesskit_root_id(), &Context::default()).unwrap(), Sense::from_bits(0u8).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_egui_src_ui_rs_1390_9_301ec2`).

## Output

```
thread 'main' (196712) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1390:9:
rect is nan in advance_cursor_after_rect
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::ui::Ui>::advance_cursor_after_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1390:9
   3: <egui::ui::Ui>::allocate_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1384:23
   4: repro_egui_src_ui_rs_1390_9_301ec2::main
             at ./src/main.rs:11:16
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
