# gdk4/src/auto/paintable.rs:14:1

- crate: gdk4 0.11.5
- kind: panic
- location: `gdk4/src/auto/paintable.rs:14:1`
- source: https://docs.rs/crate/gdk4/0.11.5/source/src/auto/paintable.rs#14
- last API call: `gdk4::Paintable::new_empty`
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
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
use gdk4::Paintable;

fn main() {
    let _ = Paintable::new_empty(-1000000000i32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_gdk4_src_auto_paintable_rs_14_1_b09b51`).

## Output

```
(process:3291192): Gdk-CRITICAL **: 17:24:31.501: gdk_paintable_new_empty: assertion 'intrinsic_width >= 0' failed
thread 'main' (3291192) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/paintable.rs:14:1:
assertion failed: !ptr.is_null()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gdk4::auto::paintable::Paintable as glib::translate::FromGlibPtrFull<*mut gdk4_sys::GdkPaintable>>::from_glib_full
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:939:17
   4: glib::translate::from_glib_full::<*mut gdk4_sys::GdkPaintable, gdk4::auto::paintable::Paintable>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:1637:14
   5: <gdk4::auto::paintable::Paintable>::new_empty
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/paintable.rs:30:13
   6: repro_gdk4_src_auto_paintable_rs_14_1_b09b51::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
