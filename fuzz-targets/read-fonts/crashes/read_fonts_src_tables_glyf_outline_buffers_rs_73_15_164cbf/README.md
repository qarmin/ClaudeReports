# read-fonts/src/tables/glyf/outline/buffers.rs:73:15

- crate: read-fonts 0.45.0
- kind: panic
- location: `read-fonts/src/tables/glyf/outline/buffers.rs:73:15`
- source: https://docs.rs/crate/read-fonts/0.45.0/source/src/tables/glyf/outline/buffers.rs#73
- last API call: `read_fonts::tables::glyf::outline::BufferLengths::packed_len[Unscaled]`
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
read-fonts = { version = "=0.45.0" }
```

`src/main.rs`:

```rust
use read_fonts::tables::glyf::outline::BufferLengths;

fn main() {
    let buffer_lengths = BufferLengths { points: 0u32, contours: 0u32, unscaled: 0u32, deltas: u32::MAX, iup: 0u32, composite_deltas: 1u32 };
    let _ = buffer_lengths.packed_len::<read_fonts::tables::glyf::outline::Unscaled>();
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_tables_glyf_outline_buffers_rs_73_15_164cbf`).

## Output

```
thread 'main' (364593) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.45.0/src/tables/glyf/outline/buffers.rs:73:15:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <read_fonts::tables::glyf::outline::buffers::BufferLengths>::packed_len::<read_fonts::tables::glyf::outline::scale::Unscaled>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/read-fonts-0.45.0/src/tables/glyf/outline/buffers.rs:73:15
   4: repro_read_fonts_src_tables_glyf_outline_buffers_rs_73_15_164cbf::main
             at ./src/main.rs:5:28
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
