# egui/src/ui.rs:114:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/ui.rs:114:9`
- last API call: `egui::Ui::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15122075744700306095

## Message

```
Top-level Ui:s should not have an UiBuilder::id_source
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
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let _ = Ui::new(Context::default(), Id::unique("".to_string()), UiBuilder::new().id(egui::accesskit_root_id()));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_ui_rs_114_9_21860d`).

## Output

```
thread 'main' (84297) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:114:9:
Top-level Ui:s should not have an UiBuilder::id_source
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::ui::Ui>::new
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:114:9
   3: repro_egui_src_ui_rs_114_9_21860d::main
             at ./src/main.rs:7:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
