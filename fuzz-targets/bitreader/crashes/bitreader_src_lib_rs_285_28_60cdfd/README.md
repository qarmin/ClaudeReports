# bitreader/src/lib.rs:285:28

- crate: https://github.com/irauta/bitreader @ 22b586b1114b589ba3fdc7b854a3ccf836a80d29
- kind: panic
- location: `bitreader/src/lib.rs:285:28`
- last API call: `bitreader::BitReader::skip`
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
bitreader = { git = "https://github.com/irauta/bitreader", rev = "22b586b1114b589ba3fdc7b854a3ccf836a80d29" }
```

`src/main.rs`:

```rust
use bitreader::BitReader;

fn main() {
    let vec = vec![0u8, 0u8, 74u8, 70u8, 73u8];
    let bit_reader = BitReader::new(&vec);
    let mut bit_reader2 = bit_reader.relative_reader_atmost(u64::MAX);
    let _ = bit_reader2.read_i64(38u8);
    let _ = bit_reader2.skip(18446744073709551614u64);
}
```

Run with `cargo run` in this directory (package `repro_bitreader_src_lib_rs_285_28_60cdfd`).

## Output

```
thread 'main' (521800) panicked at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:285:28:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <bitreader::BitReader>::skip
             at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:285:28
   4: repro_bitreader_src_lib_rs_285_28_60cdfd::main
             at ./src/main.rs:8:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
