# printpdf/src/font.rs:1677:27

- crate: printpdf 0.12.8
- kind: panic
- location: `printpdf/src/font.rs:1677:27`
- source: https://docs.rs/crate/printpdf/0.12.8/source/src/font.rs#1677
- last API call: `printpdf::font::get_normalized_widths_codes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8" }
```

`src/main.rs`:

```rust
use printpdf::font::BuiltinFont;

fn main() {
    let _ = printpdf::font::get_normalized_widths_codes(&BuiltinFont::default().get_parsed_font().unwrap(), &[(u16::MAX, 0u16), (0u16, 0u16)]);
}
```

Run with `cargo run` in this directory (package `repro_printpdf_src_font_rs_1677_27_d7ec6d`).

## Output

```
thread 'main' (254501) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/font.rs:1677:27:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: printpdf::font::get_normalized_widths_codes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/font.rs:1677:27
   4: repro_printpdf_src_font_rs_1677_27_d7ec6d::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
