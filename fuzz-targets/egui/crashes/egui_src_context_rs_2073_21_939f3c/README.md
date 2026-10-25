# egui/src/context.rs:2073:21

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/context.rs:2073:21`
- last API call: `egui::Context::cumulative_frame_nr_for`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7807046783478465198

## Message

```
cumulative_frame_nr_for failed to find the viewport "B5D0"
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
use egui::ViewportId;

fn main() {
    let context = Context::default();
    let _ = context.cumulative_frame_nr_for(ViewportId::from_hash_of(0u8));
}
```

Run with `cargo run` in this directory (package `repro_egui_src_context_rs_2073_21_939f3c`).

## Output

```
thread 'main' (83351) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:2073:21:
cumulative_frame_nr_for failed to find the viewport "B5D0"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <egui::context::Context>::cumulative_frame_nr_for::{closure#0}::{closure#1}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:2073:21
   3: <core::option::Option<u64>>::unwrap_or_else::<<egui::context::Context>::cumulative_frame_nr_for::{closure#0}::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:21
   4: <egui::context::Context>::cumulative_frame_nr_for::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:2072:18
   5: <egui::context::Context>::read::<u64, <egui::context::Context>::cumulative_frame_nr_for::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:915:9
   6: <egui::context::Context>::cumulative_frame_nr_for
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:2068:14
   7: repro_egui_src_context_rs_2073_21_939f3c::main
             at ./src/main.rs:6:21
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
