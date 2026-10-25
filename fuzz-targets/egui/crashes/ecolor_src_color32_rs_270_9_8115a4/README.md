# ecolor/src/color32.rs:270:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `ecolor/src/color32.rs:270:9`
- last API call: `ecolor::Color32::gamma_multiply`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8923734956561097233

## Message

```
factor should be finite, but was -0.0000006629608
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;

fn main() {
    let color32 = Color32::BROWN;
    let _ = color32.gamma_multiply(-6.629608e-7f32);
}
```

Run with `cargo run` in this directory (package `repro_ecolor_src_color32_rs_270_9_8115a4`).

## Output

```
thread 'main' (81188) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/color32.rs:270:9:
factor should be finite, but was -0.0000006629608
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <ecolor::color32::Color32>::gamma_multiply
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/color32.rs:270:9
   3: repro_ecolor_src_color32_rs_270_9_8115a4::main
             at ./src/main.rs:5:21
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
