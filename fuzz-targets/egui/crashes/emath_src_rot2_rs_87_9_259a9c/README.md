# emath/src/rot2.rs:87:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/rot2.rs:87:9`
- last API call: `emath::Rot2::normalized`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2679667600043294929

## Message

```
Rot2::normalized produced a non-finite result
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
use emath::Rot2;

fn main() {
    let context = Context::default();
    let painter = context.debug_painter();
    let f32_v = painter.opacity();
    let rot2 = <emath::Rot2 as ::core::ops::Mul<f32>>::mul(<emath::Rot2 as ::core::ops::Div<f32>>::div(Rot2::IDENTITY, 0.0f32), f32_v);
    let rot22 = rot2.inverse();
    let rot23 = rot22.inverse();
    let _ = rot23.normalized();
}
```

Run with `cargo run` in this directory (package `repro_emath_src_rot2_rs_87_9_259a9c`).

## Output

```
thread 'main' (86199) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rot2.rs:87:9:
Rot2::normalized produced a non-finite result
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <emath::rot2::Rot2>::normalized
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/rot2.rs:87:9
   3: repro_emath_src_rot2_rs_87_9_259a9c::main
             at ./src/main.rs:11:19
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
