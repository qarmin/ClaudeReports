# emath/src/lib.rs:197:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/lib.rs:197:9`
- last API call: `emath::RectTransform::transform_pos_clamped`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13206397296915962065

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
use egui::Context;
use egui::PointerState;
use emath::Align2;
use emath::Pos2;
use emath::RectTransform;

fn main() {
    let rect_transform = RectTransform::identity(Align2::CENTER_CENTER.align_size_within_rect(PointerState::default().velocity(), Context::default().content_rect()));
    let _ = rect_transform.transform_pos_clamped(Pos2::NAN);
}
```

Run with `cargo run` in this directory (package `repro_emath_src_lib_rs_197_9_931679`).

## Output

```
thread 'main' (225892) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:197:9:
from.start() and from.end() should not be equal
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: emath::remap_clamp::<f32, emath::range::Rangef, emath::range::Rangef>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:197:9
   3: <emath::rect_transform::RectTransform>::transform_pos_clamped
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect_transform.rs:61:13
   4: repro_emath_src_lib_rs_197_9_931679::main
             at ./src/main.rs:9:28
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
