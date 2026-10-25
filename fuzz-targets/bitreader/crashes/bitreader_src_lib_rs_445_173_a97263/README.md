# bitreader/src/lib.rs:445:173

- crate: https://github.com/irauta/bitreader @ 22b586b1114b589ba3fdc7b854a3ccf836a80d29
- kind: panic
- location: `bitreader/src/lib.rs:445:173`
- last API call: `bitreader::BitReaderError::to_string`
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
bitreader = { git = "https://github.com/irauta/bitreader", rev = "22b586b1114b589ba3fdc7b854a3ccf836a80d29" }
```

`src/main.rs`:

```rust
use bitreader::BitReader;
use bitreader::BitReaderError;

fn main() {
    let vec = vec![];
    let bit_reader = BitReader::new(&vec);
    let u64_v = bit_reader.position();
    let bit_reader_error = BitReaderError::NotEnoughData { position: 1u64, length: u64_v, requested: 0u64 };
    let _ = bit_reader_error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_bitreader_src_lib_rs_445_173_a97263`).

## Output

```
thread 'main' (521926) panicked at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:445:173:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <bitreader::BitReaderError as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:445:173
   4: <bitreader::BitReaderError as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   5: <bitreader::BitReaderError as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   6: repro_bitreader_src_lib_rs_445_173_a97263::main
             at ./src/main.rs:9:30
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
