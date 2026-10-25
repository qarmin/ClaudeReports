# read-fonts/src/tables/variations.rs:177:62

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `read-fonts/src/tables/variations.rs:177:62`
- last API call: `read_fonts::tables::variations::TupleVariationHeader::intermediate_start_tuple`
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
    let vec = vec![127u8, 127u8, 127u8, 127u8];
    let tuple_variation_header = TupleVariationHeader::read(FontData::new(&vec), 1u16).unwrap();
    let tuple_variation_header2 = tuple_variation_header.shape();
    let tuple_variation_header3 = tuple_variation_header2.shape();
    let _ = tuple_variation_header3.intermediate_start_tuple();
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_tables_variations_rs_177_62_1e9688`).

## Output

```
thread 'main' (2390048) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:177:62:
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
   4: <read_fonts::tables::variations::TupleVariationHeader>::intermediate_start_tuple::{closure#0}
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:177:62
   5: <bool>::then::<read_fonts::tables::variations::Tuple, <read_fonts::tables::variations::TupleVariationHeader>::intermediate_start_tuple::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/bool.rs:66:24
   6: <read_fonts::tables::variations::TupleVariationHeader>::intermediate_start_tuple
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/variations.rs:174:50
   7: repro_read_fonts_src_tables_variations_rs_177_62_1e9688::main
             at ./src/main.rs:9:37
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
