# allsorts-azul/src/font.rs:1213:25

- crate: printpdf 0.12.8
- kind: panic
- location: `allsorts-azul/src/font.rs:1213:25`
- source: https://docs.rs/crate/allsorts-azul/0.17.2/source/src/font.rs#1213
- last API call: `printpdf::font::ParsedFont::from_bytes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 4194324 out of range for slice of length 60
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8" }
```

`src/main.rs`:

```rust
use printpdf::font::ParsedFont;

fn main() {
    let _ = ParsedFont::from_bytes(&include_bytes!("data_0.bin").to_vec(), 1usize, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_azul_src_font_rs_1213_25_2bd0ab`).

## Output

```
thread 'main' (256086) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/allsorts-azul-0.17.2/src/font.rs:1213:25:
range start index 4194324 out of range for slice of length 60
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[u8] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <allsorts::font::Font<azul_layout::font::parsed::ManualTableProvider>>::cmap_subtable_data
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/allsorts-azul-0.17.2/src/font.rs:1213:25
   6: <azul_layout::font::parsed::ParsedFont>::from_provider::<azul_layout::font::parsed::ManualTableProvider>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/font.rs:1319:44
   7: <azul_layout::font::parsed::ParsedFont>::from_bytes_internal
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/font.rs:974:25
   8: <azul_layout::font::parsed::ParsedFont>::from_bytes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/azul-layout-0.0.16/src/font.rs:910:28
   9: <printpdf::font::parsed_font::ParsedFont>::from_bytes
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/printpdf-0.12.8/src/font.rs:96:25
  10: repro_allsorts_azul_src_font_rs_1213_25_2bd0ab::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
