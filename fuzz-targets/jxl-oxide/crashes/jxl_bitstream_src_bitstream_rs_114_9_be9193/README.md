# jxl-bitstream/src/bitstream.rs:114:9

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-bitstream/src/bitstream.rs:114:9`
- last API call: `jxl_bitstream::Bitstream::peek_bits_prefilled`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: n <= 32
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-bitstream = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_bitstream::Bitstream;

fn main() {
    let vec = vec![];
    let mut bitstream = Bitstream::new(&vec);
    let _ = bitstream.peek_bits_prefilled(1000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_jxl_bitstream_src_bitstream_rs_114_9_be9193`).

## Output

```
thread 'main' (407966) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-bitstream/src/bitstream.rs:114:9:
assertion failed: n <= 32
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <jxl_bitstream::bitstream::Bitstream>::peek_bits_prefilled
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-bitstream/src/bitstream.rs:114:9
   4: repro_jxl_bitstream_src_bitstream_rs_114_9_be9193::main
             at ./src/main.rs:6:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
