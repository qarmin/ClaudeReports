# read-fonts/src/tables/variations.rs:203:72

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `read-fonts/src/tables/variations.rs:203:72`
- last API call: `read_fonts::tables::variations::TupleVariationHeader::intermediate_tuples`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: OutOfBounds
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
```

`src/main.rs`:

```rust
use read_fonts::FontData;
use read_fonts::tables::variations::TupleVariationHeader;

fn main() {
    let vec = vec![92u8, 17u8, 227u8, 177u8];
    let font_data = FontData::new(&vec);
    let tuple_variation_header = TupleVariationHeader::read(font_data, 1u16).unwrap();
    let _ = tuple_variation_header.intermediate_tuples();
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_tables_variations_rs_203_72_a10999`).

## Output

```
thread 'main' (2390304) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:203:72:
called `Result::unwrap()` on an `Err` value: OutOfBounds
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&[font_types::raw::BigEndian<font_types::fixed::F2Dot14>], read_fonts::read::ReadError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <read_fonts::tables::variations::TupleVariationHeader>::intermediate_tuples::{closure#0}
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:203:72
   5: <bool>::then::<(read_fonts::tables::variations::Tuple, read_fonts::tables::variations::Tuple), <read_fonts::tables::variations::TupleVariationHeader>::intermediate_tuples::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:24
   6: <read_fonts::tables::variations::TupleVariationHeader>::intermediate_tuples
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:198:50
   7: repro_read_fonts_src_tables_variations_rs_203_72_a10999::main
             at ./src/main.rs:8:36
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
