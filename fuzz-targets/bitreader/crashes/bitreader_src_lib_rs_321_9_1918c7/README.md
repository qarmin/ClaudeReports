# bitreader/src/lib.rs:321:9

- crate: https://github.com/irauta/bitreader @ 22b586b1114b589ba3fdc7b854a3ccf836a80d29
- kind: panic
- location: `bitreader/src/lib.rs:321:9`
- last API call: `bitreader::BitReader::is_aligned`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to calculate the remainder with a divisor of zero
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
    let vec = vec![];
    let bit_reader = BitReader::new(&vec);
    let _ = bit_reader.is_aligned(0u32);
}
```

Run with `cargo run` in this directory (package `repro_bitreader_src_lib_rs_321_9_1918c7`).

## Output

```
thread 'main' (521838) panicked at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:321:9:
attempt to calculate the remainder with a divisor of zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_rem_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <bitreader::BitReader>::is_aligned
             at /home/rafal/.cargo/git/checkouts/bitreader-88762270f16cca93/22b586b/src/lib.rs:321:9
   4: repro_bitreader_src_lib_rs_321_9_1918c7::main
             at ./src/main.rs:6:24
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
