# lzma-rust2/src/enc/lzma2_writer.rs:162:10

- crate: https://github.com/hasenbanck/lzma-rust2 @ d6268a287ceb288114a02a5f29da28acaaaf9e52
- kind: panic
- location: `lzma-rust2/src/enc/lzma2_writer.rs:162:10`
- last API call: `lzma_rust2::LzmaOptions::get_props`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lzma-rust2 = { git = "https://github.com/hasenbanck/lzma-rust2", rev = "d6268a287ceb288114a02a5f29da28acaaaf9e52" }
```

`src/main.rs`:

```rust
use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;

fn main() {
    let mf_type = MfType::Bt4;
    let lzma_options = LzmaOptions::new(1u32, 0u32, 1000000000u32, 0u32, EncodeMode::Fast, 0u32, mf_type, 0i32);
    let _ = lzma_options.get_props();
}
```

Run with `cargo run` in this directory (package `repro_lzma_rust2_src_enc_lzma2_writer_rs_162_10_d2e95a`).

## Output

```
thread 'main' (460531) panicked at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/lzma2_writer.rs:162:10:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <lzma_rust2::enc::lzma2_writer::LzmaOptions>::get_props
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/lzma2_writer.rs:162:10
   4: repro_lzma_rust2_src_enc_lzma2_writer_rs_162_10_d2e95a::main
             at ./src/main.rs:8:26
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
