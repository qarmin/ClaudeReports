# emath/src/rect.rs:710:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/rect.rs:710:9`
- last API call: `emath::Rect::intersects_ray`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17323810304093769066

## Message

```
Debug assert: expected normalized direction, but `d` has length 0
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
use emath::Pos2;
use emath::Rect;
use epaint::MarginF32;

fn main() {
    let margin_f32 = MarginF32::ZERO;
    let rect = Rect::everything_below(0.0f32);
    let _ = rect.intersects_ray(Pos2::ZERO, margin_f32.left_top());
}
```

Run with `cargo run` in this directory (package `repro_emath_src_rect_rs_710_9_803cd5`).

## Output

```
thread 'main' (191158) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect.rs:710:9:
Debug assert: expected normalized direction, but `d` has length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <emath::rect::Rect>::intersects_ray
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect.rs:710:9
   3: repro_emath_src_rect_rs_710_9_803cd5::main
             at ./src/main.rs:8:18
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
