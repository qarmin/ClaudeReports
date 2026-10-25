# gdk4/src/auto/texture.rs:15:1

- crate: gdk4 0.11.5
- kind: panic
- location: `gdk4/src/auto/texture.rs:15:1`
- source: https://docs.rs/crate/gdk4/0.11.5/source/src/auto/texture.rs#15
- last API call: `gdk4::MemoryTextureBuilder::build`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !ptr.is_null()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5", features = ["gl", "v4_10", "v4_12", "v4_14", "v4_16", "v4_18", "v4_2", "v4_20", "v4_22", "v4_4", "v4_6", "v4_8"] }
```

`src/main.rs`:

```rust
use gdk4::MemoryTextureBuilder;

fn main() {
    let memory_texture_builder = MemoryTextureBuilder::new();
    let _ = memory_texture_builder.build();
}
```

Run with `cargo run` in this directory (package `repro_gdk4_src_auto_texture_rs_15_1_a2a2b1`).

## Output

```
(process:3350986): Gdk-CRITICAL **: 17:27:57.838: gdk_memory_texture_builder_build: assertion 'self->bytes != NULL' failed
thread 'main' (3350986) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/texture.rs:15:1:
assertion failed: !ptr.is_null()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gdk4::auto::texture::Texture as glib::translate::FromGlibPtrFull<*mut gdk4_sys::GdkTexture>>::from_glib_full
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:939:17
   4: glib::translate::from_glib_full::<*mut gdk4_sys::GdkTexture, gdk4::auto::texture::Texture>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:1637:14
   5: <gdk4::auto::memory_texture_builder::MemoryTextureBuilder>::build
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/memory_texture_builder.rs:26:18
   6: repro_gdk4_src_auto_texture_rs_15_1_a2a2b1::main
             at ./src/main.rs:5:36
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
