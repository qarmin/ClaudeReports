# ecolor/src/rgba.rs:106:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `ecolor/src/rgba.rs:106:9`
- last API call: `ecolor::Rgba::from_black_alpha`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15414159377883455484

## Message

```
a should be in the range [0, 1], but was -0.000000000000000000000000000000000000011754944
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
    let _ = Rgba::from_black_alpha(-1.1754944e-38f32);
}
```

Run with `cargo run` in this directory (package `repro_ecolor_src_rgba_rs_106_9_dcbb53`).

## Output

```
thread 'main' (12558) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/rgba.rs:106:9:
a should be in the range [0, 1], but was -0.000000000000000000000000000000000000011754944
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <ecolor::rgba::Rgba>::from_black_alpha
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/ecolor/src/rgba.rs:106:9
   3: repro_ecolor_src_rgba_rs_106_9_dcbb53::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
