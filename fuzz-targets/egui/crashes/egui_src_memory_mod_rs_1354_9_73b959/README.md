# egui/src/memory/mod.rs:1354:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/memory/mod.rs:1354:9`
- last API call: `egui::Context::set_sublayer`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14492580000984897694

## Message

```
assertion `left == right` failed: DEBUG ASSERT: Trying to set sublayers across layers of different order (Tooltip, Background), which is currently undefined behavior in egui
  left: Tooltip
 right: Background
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
use egui::LayerId;
use egui::Order;

fn main() {
    let context = Context::default();
    context.set_sublayer(LayerId::new(Order::Tooltip, Id::NULL), LayerId::background());
}
```

Run with `cargo run` in this directory (package `repro_egui_src_memory_mod_rs_1354_9_73b959`).

## Output

```
thread 'main' (201248) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/memory/mod.rs:1354:9:
assertion `left == right` failed: DEBUG ASSERT: Trying to set sublayers across layers of different order (Tooltip, Background), which is currently undefined behavior in egui
  left: Tooltip
 right: Background
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<egui::layers::Order, egui::layers::Order>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <egui::memory::Areas>::set_sublayer
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/memory/mod.rs:1354:9
   5: <egui::context::Context>::set_sublayer::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:3641:47
   6: <egui::context::Context>::memory_mut::<(), <egui::context::Context>::set_sublayer::{closure#0}>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1283:31
   7: <egui::context::Context>::write::<(), <egui::context::Context>::memory_mut<(), <egui::context::Context>::set_sublayer::{closure#0}>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:920:9
   8: <egui::context::Context>::memory_mut::<(), <egui::context::Context>::set_sublayer::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:1283:14
   9: <egui::context::Context>::set_sublayer
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/context.rs:3641:14
  10: repro_egui_src_memory_mod_rs_1354_9_73b959::main
             at ./src/main.rs:8:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
