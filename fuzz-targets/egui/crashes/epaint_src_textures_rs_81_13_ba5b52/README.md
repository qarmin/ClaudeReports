# epaint/src/textures.rs:81:13

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/textures.rs:81:13`
- last API call: `egui::load::SizedTexture::from_handle`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7097626382025028406

## Message

```
Tried freeing texture Managed(0) which is not allocated
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
use egui::load::SizedTexture;
use epaint::TextureHandle;
use epaint::TextureId;

fn main() {
    let _ = SizedTexture::from_handle(&TextureHandle::new(std::sync::Arc::new(<epaint::mutex::RwLock<epaint::TextureManager> as ::core::default::Default>::default()), TextureId::default()));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_textures_rs_81_13_ba5b52`).

## Output

```
thread 'main' (83154) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:81:13:
Tried freeing texture Managed(0) which is not allocated
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::textures::TextureManager>::free
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:81:13
   3: <epaint::texture_handle::TextureHandle as core::ops::drop::Drop>::drop
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/texture_handle.rs:27:31
   4: core::ptr::drop_glue::<epaint::texture_handle::TextureHandle>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   5: repro_epaint_src_textures_rs_81_13_ba5b52::main
             at ./src/main.rs:6:190
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
