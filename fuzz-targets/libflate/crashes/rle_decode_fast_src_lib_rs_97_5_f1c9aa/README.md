# rle-decode-fast/src/lib.rs:97:5

- crate: https://github.com/sile/libflate @ fd6cfb6b21328ee81ff95178aed45d64a94a86a3
- kind: panic
- location: `rle-decode-fast/src/lib.rs:97:5`
- source: https://docs.rs/crate/rle-decode-fast/1.0.3/source/src/lib.rs#97
- last API call: `libflate_lz77::Lz77Decoder::decode`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to repeat fragment of size 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
libflate = { git = "https://github.com/sile/libflate", rev = "fd6cfb6b21328ee81ff95178aed45d64a94a86a3" }
libflate_lz77 = { git = "https://github.com/sile/libflate", rev = "fd6cfb6b21328ee81ff95178aed45d64a94a86a3" }
```

`src/main.rs`:

```rust
use libflate_lz77::Code;
use libflate_lz77::Lz77Decoder;

fn main() {
    let mut lz77_decoder = Lz77Decoder::default();
    let _ = lz77_decoder.decode(Code::Pointer { length: 0u16, backward_distance: 0u16 });
}
```

Run with `cargo run` in this directory (package `repro_rle_decode_fast_src_lib_rs_97_5_f1c9aa`).

## Output

```
thread 'main' (583124) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rle-decode-fast-1.0.3/src/lib.rs:97:5:
attempt to repeat fragment of size 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: rle_decode_fast::lookbehind_length_fail
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rle-decode-fast-1.0.3/src/lib.rs:97:5
   4: rle_decode_fast::rle_decode::<u8>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rle-decode-fast-1.0.3/src/lib.rs:41:9
   5: <libflate_lz77::Lz77Decoder>::decode
             at /home/rafal/.cargo/git/checkouts/libflate-56967bc39b4ad21b/fd6cfb6/libflate_lz77/src/lib.rs:186:17
   6: repro_rle_decode_fast_src_lib_rs_97_5_f1c9aa::main
             at ./src/main.rs:6:26
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
