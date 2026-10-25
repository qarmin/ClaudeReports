# rawler/src/bits.rs:231:23

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/bits.rs:231:23`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/bits.rs#231
- last API call: `rawler::bits::LookupTable::new_with_bits`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::bits::LookupTable;

fn main() {
    let _ = LookupTable::new_with_bits(&[0u16], 10000u32);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_bits_rs_231_23_dcf317`).

## Output

```
thread 'main' (430819) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/bits.rs:231:23:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rawler::bits::LookupTable>::new_with_bits
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/bits.rs:231:23
   4: repro_rawler_src_bits_rs_231_23_dcf317::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
