# emath/src/lib.rs:170:5

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/lib.rs:170:5`
- last API call: `emath::RectTransform::transform_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12917710754109487109

## Message

```
from.start() and from.end() should not be equal
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
use egui::PointerState;
use emath::Align2;
use emath::Pos2;
use emath::RectTransform;

fn main() {
    let align2 = Align2::CENTER_TOP;
    let rect = align2.anchor_size(Pos2 { x: 0.0f32, y: 0.0f32 }, PointerState::default().direction());
    let rect2 = rect.with_min_y(0.0f32);
    let _ = RectTransform::identity(rect2).transform_rect(rect);
}
```

Run with `cargo run` in this directory (package `repro_emath_src_lib_rs_170_5_fee24b`).

## Output

```
thread 'main' (192768) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:170:5:
from.start() and from.end() should not be equal
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: emath::remap::<f32, emath::range::Rangef, emath::range::Rangef>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:170:5
   3: <emath::rect_transform::RectTransform>::transform_pos
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect_transform.rs:44:13
   4: <emath::rect_transform::RectTransform>::transform_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect_transform.rs:52:23
   5: repro_emath_src_lib_rs_170_5_fee24b::main
             at ./src/main.rs:10:44
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
