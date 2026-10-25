# epaint/src/textures.rs:92:13

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/textures.rs:92:13`
- last API call: `epaint::TextureManager::retain`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4390424630213566281

## Message

```
Tried retaining texture Managed(0) which is not allocated
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::TextureId;
use epaint::TextureManager;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.retain(TextureId::default());
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_textures_rs_92_13_7a0022`).

## Output

```
thread 'main' (72234) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:92:13:
Tried retaining texture Managed(0) which is not allocated
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::textures::TextureManager>::retain
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:92:13
   3: repro_epaint_src_textures_rs_92_13_7a0022::main
             at ./src/main.rs:6:21
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
