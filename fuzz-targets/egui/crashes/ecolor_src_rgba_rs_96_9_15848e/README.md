# ecolor/src/rgba.rs:96:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `ecolor/src/rgba.rs:96:9`
- last API call: `ecolor::Rgba::from_luminance_alpha`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8590627100265645485

## Message

```
a should be in the range [0, 1], but was -340282350000000000000000000000000000000
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
use ecolor::Rgba;

fn main() {
    let _ = Rgba::from_luminance_alpha(0.0f32, f32::MIN);
}
```

Run with `cargo run` in this directory (package `repro_ecolor_src_rgba_rs_96_9_15848e`).

## Output

```
thread 'main' (192918) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/rgba.rs:96:9:
a should be in the range [0, 1], but was -340282350000000000000000000000000000000
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <ecolor::rgba::Rgba>::from_luminance_alpha
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/rgba.rs:96:9
   3: repro_ecolor_src_rgba_rs_96_9_15848e::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
