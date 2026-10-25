# egui/src/widgets/value_format.rs:207:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/widgets/value_format.rs:207:9`
- last API call: `egui::DragValue::hexadecimal`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16493467968794039786

## Message

```
ValueFormat::hexadecimal: `min_width` must be greater than 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::DragValue;

fn main() {
    let drag_value = DragValue::from_get_set(|_| 0 as f64);
    let drag_value2 = drag_value.max_decimals(0usize);
    let _ = drag_value2.hexadecimal(0usize, false, false);
}
```

Run with `cargo run` in this directory (package `repro_egui_src_widgets_value_format_rs_207_9_7901ea`).

## Output

```
thread 'main' (192470) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/value_format.rs:207:9:
ValueFormat::hexadecimal: `min_width` must be greater than 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::widgets::value_format::ValueFormat>::hexadecimal
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/value_format.rs:207:9
   3: <egui::widgets::drag_value::DragValue>::hexadecimal
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/widgets/drag_value.rs:370:35
   4: repro_egui_src_widgets_value_format_rs_207_9_7901ea::main
             at ./src/main.rs:6:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
