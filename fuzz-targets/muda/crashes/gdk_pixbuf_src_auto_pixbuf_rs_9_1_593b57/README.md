# gdk-pixbuf/src/auto/pixbuf.rs:9:1

- crate: https://github.com/tauri-apps/muda @ 33dfdf7c6daceed1dddd73b658e0e4c912685f36
- kind: panic
- location: `gdk-pixbuf/src/auto/pixbuf.rs:9:1`
- source: https://docs.rs/crate/gdk-pixbuf/0.18.5/source/src/auto/pixbuf.rs#9
- last API call: `muda::Icon::from_rgba`
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
muda = { git = "https://github.com/tauri-apps/muda", rev = "33dfdf7c6daceed1dddd73b658e0e4c912685f36", features = ["common-controls-v6", "gtk3", "gtk4", "serde", "snapshot"] }
```

`src/main.rs`:

```rust
use muda::Icon;

fn main() {
    let _ = Icon::from_rgba(vec![], 0u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_gdk_pixbuf_src_auto_pixbuf_rs_9_1_593b57`).

## Output

```
(process:516901): GdkPixbuf-CRITICAL **: 22:21:04.820: gdk_pixbuf_new_from_bytes: assertion 'width > 0' failed
thread 'main' (516901) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk-pixbuf-0.22.0/src/auto/pixbuf.rs:9:1:
assertion failed: !ptr.is_null()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gdk_pixbuf::auto::pixbuf::Pixbuf as glib::translate::FromGlibPtrFull<*mut gdk_pixbuf_sys::GdkPixbuf>>::from_glib_full
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:939:17
   4: glib::translate::from_glib_full::<*mut gdk_pixbuf_sys::GdkPixbuf, gdk_pixbuf::auto::pixbuf::Pixbuf>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:1637:14
   5: <gdk_pixbuf::auto::pixbuf::Pixbuf>::from_bytes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk-pixbuf-0.22.0/src/auto/pixbuf.rs:50:13
   6: <muda::platform_impl::platform::icon::PlatformIcon>::from_rgba
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/platform_impl/gtk4/icon.rs:28:22
   7: <muda::icon::Icon>::from_rgba
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/icon.rs:139:24
   8: repro_gdk_pixbuf_src_auto_pixbuf_rs_9_1_593b57::main
             at ./src/main.rs:4:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
