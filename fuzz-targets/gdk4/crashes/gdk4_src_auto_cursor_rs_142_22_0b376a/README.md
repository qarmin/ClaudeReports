# gdk4/src/auto/cursor.rs:142:22

- crate: gdk4 0.11.5
- kind: panic
- location: `gdk4/src/auto/cursor.rs:142:22`
- source: https://docs.rs/crate/gdk4/0.11.5/source/src/auto/cursor.rs#142
- last API call: `gdk4::builders::CursorBuilder::build`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
property 'hotspot-y' of type 'GdkCursor' can't be set from given value, it is invalid or out of range
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
use gdk4::Cursor;

fn main() {
    let cursor_builder = Cursor::builder();
    let cursor_builder2 = cursor_builder.hotspot_y(-1000000000i32);
    let _ = cursor_builder2.build();
}
```

Run with `cargo run` in this directory (package `repro_gdk4_src_auto_cursor_rs_142_22_0b376a`).

## Output

```
thread 'main' (3295172) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/cursor.rs:142:22:
property 'hotspot-y' of type 'GdkCursor' can't be set from given value, it is invalid or out of range
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: glib::object::validate_property_type
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:3311:13
   3: <glib::object::Object>::new_internal
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:1514:21
   4: <glib::object::Object>::with_mut_values
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:1454:18
   5: <glib::object::ObjectBuilder<gdk4::auto::cursor::Cursor>>::build
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/object.rs:1666:22
   6: <gdk4::auto::cursor::CursorBuilder>::build
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/cursor.rs:142:22
   7: repro_gdk4_src_auto_cursor_rs_142_22_0b376a::main
             at ./src/main.rs:6:29
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
