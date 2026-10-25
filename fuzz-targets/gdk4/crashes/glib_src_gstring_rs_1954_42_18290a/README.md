# glib/src/gstring.rs:1954:42

- crate: gdk4 0.11.5
- kind: panic
- location: `glib/src/gstring.rs:1954:42`
- source: https://docs.rs/crate/glib/0.22.10/source/src/gstring.rs#1954
- last API call: `gdk4::RGBA::from_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(274)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <gdk4::RGBA as ::core::str::FromStr>::from_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"NaN\" viewBox=\"0 0 114 0\"><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><g transform=\"rotate(198) scale(153) tr\0anslate(191 NaN)\" clip-path=\"url(#c)\"><path d=\"M1e9 -1 L161 0 C1 NaN 31 1e9 188 -1 Z\" stroke=\"transparent\" str");
}
```

Run with `cargo run` in this directory (package `repro_glib_src_gstring_rs_1954_42_18290a`).

## Output

```
thread 'main' (3343837) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/gstring.rs:1954:42:
called `Result::unwrap()` on an `Err` value: GStrInteriorNulError(274)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), glib::gstring::GStrInteriorNulError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <glib::gstring::GString as core::convert::From<&str>>::from
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/gstring.rs:1954:42
   5: <&str as glib::gstring::IntoGStr>::run_with_gstr::<core::result::Result<gdk4::auto::rgba::RGBA, glib::error::BoolError>, <gdk4::auto::rgba::RGBA>::parse<&str>::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glib-0.22.10/src/gstring.rs:2516:15
   6: <gdk4::auto::rgba::RGBA>::parse::<&str>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/rgba.rs:205:15
   7: <gdk4::auto::rgba::RGBA as core::str::traits::FromStr>::from_str
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gdk4-0.11.5/src/rgba.rs:244:9
   8: repro_glib_src_gstring_rs_1954_42_18290a::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
