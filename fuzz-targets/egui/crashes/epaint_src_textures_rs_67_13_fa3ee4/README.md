# epaint/src/textures.rs:67:13

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/textures.rs:67:13`
- last API call: `epaint::TextureManager::set`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17620270099997119419

## Message

```
Tried setting texture Managed(0) which is not allocated
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
use epaint::ColorImage;
use epaint::ImageData;
use epaint::ImageDelta;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.set(TextureManager::default().alloc("".to_string(), ImageData::Color(std::sync::Arc::new(ColorImage::example())), TextureOptions::NEAREST_REPEAT), ImageDelta { image: ImageData::from(ColorImage::default()), options: TextureOptions::NEAREST_MIRRORED_REPEAT, pos: None });
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_textures_rs_67_13_fa3ee4`).

## Output

```
thread 'main' (197221) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:67:13:
Tried setting texture Managed(0) which is not allocated
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::textures::TextureManager>::set
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:67:13
   3: repro_epaint_src_textures_rs_67_13_fa3ee4::main
             at ./src/main.rs:9:21
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
