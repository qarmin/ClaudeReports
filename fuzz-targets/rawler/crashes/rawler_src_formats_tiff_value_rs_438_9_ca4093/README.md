# rawler/src/formats/tiff/value.rs:438:9

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/formats/tiff/value.rs:438:9`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/formats/tiff/value.rs#438
- last API call: `rawler::formats::tiff::Value::get_data`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Unable to call get_data() on this value type
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::formats::tiff::CompressionMethod;
use rawler::formats::tiff::Value;

fn main() {
    let value = Value::from(CompressionMethod::Huffman);
    let _ = value.get_data();
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_formats_tiff_value_rs_438_9_ca4093`).

## Output

```
thread 'main' (431633) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/tiff/value.rs:438:9:
Unable to call get_data() on this value type
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rawler::formats::tiff::value::Value>::get_data
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/formats/tiff/value.rs:438:9
   3: repro_rawler_src_formats_tiff_value_rs_438_9_ca4093::main
             at ./src/main.rs:6:19
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
