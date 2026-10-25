# egui/src/ui.rs:766:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/ui.rs:766:9`
- last API call: `egui::Ui::set_min_width`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7133804011497675008

## Message

```
Negative width makes no sense, but got: -0.000000000000000000000000000000967
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
    let mut ui = Ui::new(Context::default(), Id::unique("".to_string()), UiBuilder::new());
    ui.set_min_width(-9.67e-31f32);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_ui_rs_766_9_00cb86`).

## Output

```
thread 'main' (192273) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:766:9:
Negative width makes no sense, but got: -0.000000000000000000000000000000967
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::ui::Ui>::set_min_width
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:766:9
   3: repro_egui_src_ui_rs_766_9_00cb86::main
             at ./src/main.rs:8:8
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
