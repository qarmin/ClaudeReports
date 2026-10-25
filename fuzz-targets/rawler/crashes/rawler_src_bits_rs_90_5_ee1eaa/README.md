# rawler/src/bits.rs:90:5

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/bits.rs:90:5`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/bits.rs#90
- last API call: `rawler::bits::Endian::read_i8`
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
use rawler::bits::Endian;

fn main() {
    let endian = Endian::default();
    let _ = endian.read_i8(&vec![], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_bits_rs_90_5_ee1eaa`).

## Output

```
thread 'main' (430783) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/bits.rs:90:5:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <rawler::bits::Endian>::read_i8
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/bits.rs:90:5
   4: repro_rawler_src_bits_rs_90_5_ee1eaa::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
