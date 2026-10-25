# emath/src/rect.rs:742:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/rect.rs:742:9`
- last API call: `emath::Rect::intersects_ray_from_center`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8623240942788527320

## Message

```
expected normalized direction, but `d` has length 0
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
use emath::Pos2;
use emath::Rect;
use emath::Vec2;

fn main() {
    let rect = Rect::from_min_max(Pos2 { x: 0.0f32, y: 0.0f32 }, Pos2::ZERO);
    let _ = rect.intersects_ray_from_center(Vec2::default());
}
```

Run with `cargo run` in this directory (package `repro_emath_src_rect_rs_742_9_4e6257`).

## Output

```
thread 'main' (191367) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect.rs:742:9:
expected normalized direction, but `d` has length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <emath::rect::Rect>::intersects_ray_from_center
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rect.rs:742:9
   3: repro_emath_src_rect_rs_742_9_4e6257::main
             at ./src/main.rs:7:18
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
