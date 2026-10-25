# egui/src/text_selection/text_cursor_state.rs:485:5

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/text_selection/text_cursor_state.rs:485:5`
- last API call: `egui::text_selection::text_cursor_state::slice_char_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 5583885540638073863

## Message

```
Invalid range, start must be less than end, but start = 1, end = 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::text::CharIndex;

fn main() {
    let _ = egui::text_selection::text_cursor_state::slice_char_range("", CharIndex::default().saturating_add(1usize)..CharIndex::default());
}
```

Run with `cargo run` in this directory (package `repro_egui_src_text_selection_text_cursor_state_rs_485_5_b651a8`).

## Output

```
thread 'main' (193551) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/text_selection/text_cursor_state.rs:485:5:
Invalid range, start must be less than end, but start = 1, end = 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: egui::text_selection::text_cursor_state::slice_char_range
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/text_selection/text_cursor_state.rs:485:5
   3: repro_egui_src_text_selection_text_cursor_state_rs_485_5_b651a8::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
