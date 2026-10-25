# ecolor/src/color32.rs:306:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `ecolor/src/color32.rs:306:9`
- last API call: `ecolor::Color32::linear_multiply`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12309332547518115881

## Message

```
factor should be finite, but was -340282350000000000000000000000000000000
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
    let _ = Color32::from_rgba_unmultiplied(0u8, 0u8, 0u8, 0u8).linear_multiply(f32::MIN);
}
```

Run with `cargo run` in this directory (package `repro_ecolor_src_color32_rs_306_9_47f9ba`).

## Output

```
thread 'main' (52803) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/color32.rs:306:9:
factor should be finite, but was -340282350000000000000000000000000000000
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <ecolor::color32::Color32>::linear_multiply
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/color32.rs:306:9
   3: repro_ecolor_src_color32_rs_306_9_47f9ba::main
             at ./src/main.rs:4:65
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
