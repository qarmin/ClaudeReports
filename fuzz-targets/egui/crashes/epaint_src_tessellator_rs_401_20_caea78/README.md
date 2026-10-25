# epaint/src/tessellator.rs:401:20

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/tessellator.rs:401:20`
- last API call: `epaint::tessellator::Path::add_open_points`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3022733346006319739

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
use emath::Pos2;
use emath::Rect;
use epaint::tessellator::Path;

fn main() {
    let pos2 = Pos2::default();
    let mut path = Path::default();
    path.add_open_points(&[pos2, Rect::everything_left_of(0.0f32).left_top(), pos2]);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_tessellator_rs_401_20_caea78`).

## Output

```
thread 'main' (243335) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:401:20:
Comparing NaN vectors ([NaN NaN] and [0.0 0.0]). A NaN is not even equal to itself, which leads to very confusing bugs.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <emath::vec2::Vec2 as core::cmp::PartialEq>::eq
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/vec2.rs:353:9
   3: <epaint::tessellator::Path>::add_open_points
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:401:20
   4: repro_epaint_src_tessellator_rs_401_20_caea78::main
             at ./src/main.rs:8:10
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
