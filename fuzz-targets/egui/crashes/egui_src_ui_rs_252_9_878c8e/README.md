# egui/src/ui.rs:252:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/ui.rs:252:9`
- last API call: `egui::Ui::place[Hyperlink]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14629868232204498756

## Message

```
max_rect is NaN: [[-inf -inf] - [NaN 0.0]]
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
use egui::Hyperlink;
use egui::Ui;
use egui::UiBuilder;
use egui::ViewportId;
use emath::Rect;

fn main() {
    let context = Context::default();
    let mut ui = Ui::new(context.clone(), ViewportId::ROOT.root_ui_id(), UiBuilder::default());
    let _ = ui.place(Rect::everything_above(0.0f32).with_max_x(f32::NAN), Hyperlink::new("".to_string()));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_ui_rs_252_9_878c8e`).

## Output

```
thread 'main' (225366) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:252:9:
max_rect is NaN: [[-inf -inf] - [NaN 0.0]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::ui::Ui>::new_child
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:252:9
   3: <egui::ui::Ui>::place::<egui::widgets::hyperlink::Hyperlink>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:1679:14
   4: repro_egui_src_ui_rs_252_9_878c8e::main
             at ./src/main.rs:11:16
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
