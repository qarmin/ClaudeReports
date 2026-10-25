# glib/src/translate.rs:564:56

- crate: gdk4 0.11.5
- kind: panic
- location: `glib/src/translate.rs:564:56`
- source: https://docs.rs/crate/glib/0.22.10/source/src/translate.rs#564
- last API call: `gdk4::Texture::from_resource`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
use gdk4::Texture;

fn main() {
    let _ = Texture::from_resource("\0");
}
```

Run with `cargo run` in this directory (package `repro_glib_src_translate_rs_564_56_ed2e26`).

## Output

```
thread 'main' (3331184) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:564:56:
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), glib::gstring::GStrInteriorNulError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <str as glib::translate::ToGlibPtr<*const i8>>::to_glib_none
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/translate.rs:564:56
   5: <gdk4::auto::texture::Texture>::from_resource
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/auto/texture.rs:92:31
   6: repro_glib_src_translate_rs_564_56_ed2e26::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
