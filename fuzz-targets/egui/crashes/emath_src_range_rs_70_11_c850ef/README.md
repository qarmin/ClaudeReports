# emath/src/range.rs:70:11

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/range.rs:70:11`
- last API call: `emath::Rangef::clamp`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2775259369190310889

## Message

```
min > max, or either was NaN. min = -0.0, max = -0.00024414063
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
use emath::Rangef;

fn main() {
    let rangef = <emath::Rangef as ::core::ops::Mul<f32>>::mul(Rangef::new(0.0f32, 1.0f32), -0.00024414063f32);
    let _ = rangef.clamp(0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_emath_src_range_rs_70_11_c850ef`).

## Output

```
thread 'main' (162111) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:1605:9:
min > max, or either was NaN. min = -0.0, max = -0.00024414063
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <f32>::clamp::do_panic::runtime
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:172:21
   3: <f32>::clamp::do_panic
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:2636:9
   4: <f32>::clamp
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:177:9
   5: <emath::range::Rangef>::clamp
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/range.rs:70:11
   6: repro_emath_src_range_rs_70_11_c850ef::main
             at ./src/main.rs:5:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
