# epaint/src/tessellator.rs:403:27

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/tessellator.rs:403:27`
- last API call: `epaint::tessellator::Path::add_open_points`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2293221448008987650

## Message

```
Comparing NaN vectors ([NaN NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
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
use egui::AreaState;
use emath::Pos2;
use emath::Rect;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    path.add_open_points(&[AreaState::default().left_top_pos(), Pos2::new(0.0f32, 1.0f32), Rect::everything_below(0.0f32).right_center()]);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_tessellator_rs_403_27_85e63c`).

## Output

```
thread 'main' (228742) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:403:27:
Comparing NaN vectors ([NaN NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <emath::vec2::Vec2 as core::cmp::PartialEq>::eq
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/vec2.rs:353:9
   3: <epaint::tessellator::Path>::add_open_points
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:403:27
   4: repro_epaint_src_tessellator_rs_403_27_85e63c::main
             at ./src/main.rs:8:10
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
