# epaint/src/shapes/shape.rs:561:5

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/shape.rs:561:5`
- last API call: `epaint::Shape::dashed_line_with_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17092829282324132892

## Message

```
assertion `left == right` failed: Mismatched dash and gap lengths, got dash_lengths: 0, gap_lengths: 1
  left: 0
 right: 1
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
use egui::Visuals;
use epaint::Shape;

fn main() {
    let _ = Shape::dashed_line_with_offset(&[], Visuals::default().window_stroke(), &[], &[0.0f32], 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_shape_rs_561_5_99590e`).

## Output

```
thread 'main' (11793) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/shape.rs:561:5:
assertion `left == right` failed: Mismatched dash and gap lengths, got dash_lengths: 0, gap_lengths: 1
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: epaint::shapes::shape::dashes_from_line
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/shape.rs:561:5
   5: <epaint::shapes::shape::Shape>::dashed_line_with_offset::<epaint::stroke::Stroke>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/shape.rs:205:9
   6: repro_epaint_src_shapes_shape_rs_561_5_99590e::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
