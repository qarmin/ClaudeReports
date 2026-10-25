# egui/src/layout.rs:84:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layout.rs:84:9`
- last API call: `egui::Ui::shrink_height_to_current`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 5913839224110703206

## Message

```
cursor has Nan: [[0.0 NaN] - [10000.0 inf]]
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
    let mut ui = Ui::new(Context::default(), Id::new("".to_string()), UiBuilder::default());
    ui.add_space(f32::NAN);
    ui.shrink_height_to_current();
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layout_rs_84_9_0c5b65`).

## Output

```
thread 'main' (298303) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:84:9:
cursor has Nan: [[0.0 NaN] - [10000.0 inf]]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::layout::Region>::sanity_check
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:84:9
   3: <egui::layout::Layout>::next_frame_ignore_wrap
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:583:16
   4: <egui::layout::Layout>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layout.rs:660:26
   5: <egui::placer::Placer>::next_widget_space_ignore_wrap_justify
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:228:14
   6: <egui::placer::Placer>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/placer.rs:249:25
   7: <egui::ui::Ui>::set_max_height
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:751:21
   8: <egui::ui::Ui>::shrink_height_to_current
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/ui.rs:819:14
   9: repro_egui_src_layout_rs_84_9_0c5b65::main
             at ./src/main.rs:9:8
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
