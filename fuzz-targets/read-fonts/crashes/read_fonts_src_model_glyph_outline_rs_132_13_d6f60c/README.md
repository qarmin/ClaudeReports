# read-fonts/src/model/glyph/outline.rs:132:13

- crate: read-fonts 0.45.0
- kind: panic
- location: `read-fonts/src/model/glyph/outline.rs:132:13`
- source: https://docs.rs/crate/read-fonts/0.45.0/source/src/model/glyph/outline.rs#132
- last API call: `read_fonts::model::glyph::outline::SvgPen::move_to`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Formatting argument out of range
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { version = "=0.45.0" }
```

`src/main.rs`:

```rust
use read_fonts::model::glyph::outline::SvgPen;

fn main() {
    let mut svg_pen = SvgPen::with_precision(1000000000usize);
    <read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen>::move_to(&mut svg_pen, 0.0f32, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_model_glyph_outline_rs_132_13_d6f60c`).

## Output

```
thread 'main' (366206) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.45.0/src/model/glyph/outline.rs:132:13:
Formatting argument out of range
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <core::fmt::rt::Argument>::from_usize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:132:13
   3: <read_fonts::model::glyph::outline::SvgPen as read_fonts::model::glyph::outline::OutlinePen>::move_to
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.45.0/src/model/glyph/outline.rs:132:13
   4: repro_read_fonts_src_model_glyph_outline_rs_132_13_d6f60c::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
