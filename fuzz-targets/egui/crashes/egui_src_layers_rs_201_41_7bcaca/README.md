# egui/src/layers.rs:201:41

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `egui/src/layers.rs:201:41`
- last API call: `egui::layers::PaintList::transform_after_rounding_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12624911350955450759

## Message

```
range start index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::Id;
use egui::LayerId;
use egui::Order;
use egui::layers::PaintList;
use egui::layers::ShapeIdx;
use emath::TSTransform;
use epaint::Margin;
use epaint::MarginF32;
use epaint::PathStroke;

fn main() {
    let context = Context::default();
    let f32_v = emath::easing::quadratic_out(0.0f32);
    let mut paint_list = PaintList::default();
    paint_list.transform_after_rounding_range(ShapeIdx(1usize), context.layer_painter(LayerId::new(Order::Background, Id::new(0u32))).line(vec![], PathStroke::NONE), TSTransform { scaling: f32_v, translation: MarginF32::from(Margin::symmetric(0i8, 0i8)).right_bottom() });
}
```

Run with `cargo run` in this directory (package `repro_egui_src_layers_rs_201_41_7bcaca`).

## Output

```
thread 'main' (193388) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layers.rs:201:41:
range start index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[epaint::ClippedShape]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:427:13
   4: <[epaint::ClippedShape] as core::ops::index::IndexMut<core::ops::range::Range<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<epaint::ClippedShape> as core::ops::index::IndexMut<core::ops::range::Range<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <egui::layers::PaintList>::transform_after_rounding_range
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/egui/src/layers.rs:201:41
   7: repro_egui_src_layers_rs_201_41_7bcaca::main
             at ./src/main.rs:16:16
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
