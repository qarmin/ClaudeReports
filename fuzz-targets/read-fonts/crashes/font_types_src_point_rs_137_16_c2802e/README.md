# font-types/src/point.rs:137:16

- crate: read-fonts 0.45.0
- kind: panic
- location: `font-types/src/point.rs:137:16`
- source: https://docs.rs/crate/font-types/0.12.6/source/src/point.rs#137
- last API call: `read_fonts::tables::gvar::GlyphDelta::apply_scalar[i32]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { version = "=0.45.0" }
font-types = { version = "=0.12.6" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::TupleDelta>::new(0u16, 0i32, 1000000000i32).apply_scalar::<i32>(<font_types::Fixed as font_types::Scalar>::from_raw([0u8, 10u8, 0u8, 0u8]));
}
```

Run with `cargo run` in this directory (package `repro_font_types_src_point_rs_137_16_c2802e`).

## Output

```
thread 'main' (367071) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/font-types-0.12.6/src/point.rs:137:16:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32 as core::ops::arith::Mul>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:350:45
   4: <font_types::point::Point<i32> as core::ops::arith::Mul<i32>>::mul
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/font-types-0.12.6/src/point.rs:137:16
   5: <read_fonts::tables::gvar::GlyphDelta>::apply_scalar::<i32>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.45.0/src/tables/gvar.rs:288:9
   6: repro_font_types_src_point_rs_137_16_c2802e::main
             at ./src/main.rs:2:130
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
