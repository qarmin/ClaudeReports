# gdk4/src/auto/display.rs:23:1

- crate: gdk4 0.11.5
- kind: panic
- location: `gdk4/src/auto/display.rs:23:1`
- source: https://docs.rs/crate/gdk4/0.11.5/source/src/auto/display.rs#23
- last API call: `gdk4::DmabufTextureBuilder::display`
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
use gdk4::DmabufTextureBuilder;

fn main() {
    let dmabuf_texture_builder = DmabufTextureBuilder::new();
    let dmabuf_texture_builder2 = dmabuf_texture_builder.clone().set_stride(0u32, 0u32);
    let dmabuf_texture_builder3 = dmabuf_texture_builder2.clone().set_stride(0u32, 1u32);
    let _ = dmabuf_texture_builder3.display();
}
```

Run with `cargo run` in this directory (package `repro_gdk4_src_auto_display_rs_23_1_eb3cce`).

## Output

```
(process:3341033): GLib-GObject-CRITICAL **: 17:27:34.137: g_object_ref: assertion 'G_IS_OBJECT (object)' failed
thread 'main' (3341033) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/display.rs:23:1:
assertion failed: !ptr.is_null()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gdk4::auto::display::Display as glib::translate::FromGlibPtrNone<*mut gdk4_sys::GdkDisplay>>::from_glib_none
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:911:17
   4: glib::translate::from_glib_none::<*mut gdk4_sys::GdkDisplay, gdk4::auto::display::Display>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:1628:14
   5: <gdk4::auto::dmabuf_texture_builder::DmabufTextureBuilder>::display
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/dmabuf_texture_builder.rs:44:13
   6: repro_gdk4_src_auto_display_rs_23_1_eb3cce::main
             at ./src/main.rs:7:37
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
(process:3341033): GLib-GObject-CRITICAL **: 17:27:34.195: g_object_unref: assertion 'G_IS_OBJECT (object)' failed
```
