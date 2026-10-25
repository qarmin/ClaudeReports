# rawler/src/pumps.rs:345:5

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/pumps.rs:345:5`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/pumps.rs#345
- last API call: `rawler::decompressors::ljpeg::LjpegDecompressor::new_full`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::decompressors::ljpeg::LjpegDecompressor;

fn main() {
    let vec = vec![];
    let _ = LjpegDecompressor::new_full(&vec, false, false);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_pumps_rs_345_5_adcbcb`).

## Output

```
thread 'main' (431784) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/pumps.rs:345:5:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <rawler::pumps::ByteStream>::peek_u8
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/pumps.rs:345:5
   4: <rawler::pumps::ByteStream>::get_u8
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/pumps.rs:349:20
   5: <rawler::decompressors::ljpeg::LjpegDecompressor>::get_next_marker
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/decompressors/ljpeg/mod.rs:254:24
   6: <rawler::decompressors::ljpeg::LjpegDecompressor>::new_full
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/decompressors/ljpeg/mod.rs:182:8
   7: repro_rawler_src_pumps_rs_345_5_adcbcb::main
             at ./src/main.rs:5:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
