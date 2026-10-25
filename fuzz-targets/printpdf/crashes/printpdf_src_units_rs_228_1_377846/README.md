# printpdf/src/units.rs:228:1

- crate: printpdf 0.12.8
- kind: panic
- location: `printpdf/src/units.rs:228:1`
- source: https://docs.rs/crate/printpdf/0.12.8/source/src/units.rs#228
- last API call: `printpdf::units::Px::sub<printpdf::units::Px>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8" }
```

`src/main.rs`:

```rust
use printpdf::units::Px;

fn main() {
    let _ = <printpdf::units::Px as ::core::ops::Sub<printpdf::units::Px>>::sub(Px(0usize), Px(1usize));
}
```

Run with `cargo run` in this directory (package `repro_printpdf_src_units_rs_228_1_377846`).

## Output

```
thread 'main' (252989) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/units.rs:228:1:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <printpdf::units::Px as core::ops::arith::Sub>::sub
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/units.rs:159:24
   4: repro_printpdf_src_units_rs_228_1_377846::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
