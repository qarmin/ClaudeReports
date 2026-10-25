# rawler/src/formats/tiff/value.rs:242:32

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/formats/tiff/value.rs:242:32`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/formats/tiff/value.rs#242
- last API call: `rawler::formats::tiff::Rational::from<f32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Rational numerator overflows u32: TryFromBigIntError { original: () }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::formats::tiff::Rational;

fn main() {
    let _ = Rational::from(54016850000000.0f32);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_formats_tiff_value_rs_242_32_51e9ac`).

## Output

```
thread 'main' (431156) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/tiff/value.rs:242:32:
Rational numerator overflows u32: TryFromBigIntError { original: () }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<u32, num_bigint::TryFromBigIntError<()>>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <rawler::formats::tiff::value::Rational as core::convert::From<f32>>::from
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/tiff/value.rs:242:32
   5: repro_rawler_src_formats_tiff_value_rs_242_32_51e9ac::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
