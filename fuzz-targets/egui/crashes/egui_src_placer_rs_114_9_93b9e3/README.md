# egui/src/placer.rs:114:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/placer.rs:114:9`
- last API call: `egui::Ui::allocate_response`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2995802086471728350

## Message

```
Negative child size: [-1.0 0.0]
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
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Vec2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::new(0u8), UiBuilder::default());
    let _ = ui.allocate_response(Vec2::LEFT, Sense::CLICK);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_placer_rs_114_9_93b9e3`).

## Output

```
thread 'main' (77735) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:114:9:
Negative child size: [-1.0 0.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::placer::Placer>::next_space
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:114:9
   3: <egui::ui::Ui>::allocate_space_impl
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1366:38
   4: <egui::ui::Ui>::allocate_space
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1317:25
   5: <egui::ui::Ui>::allocate_response
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1265:31
   6: repro_egui_src_placer_rs_114_9_93b9e3::main
             at ./src/main.rs:10:16
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
