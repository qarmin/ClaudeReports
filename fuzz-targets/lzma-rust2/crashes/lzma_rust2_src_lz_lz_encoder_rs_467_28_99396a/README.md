# lzma-rust2/src/lz/lz_encoder.rs:467:28

- crate: https://github.com/hasenbanck/lzma-rust2 @ d6268a287ceb288114a02a5f29da28acaaaf9e52
- kind: panic
- location: `lzma-rust2/src/lz/lz_encoder.rs:467:28`
- last API call: `lzma_rust2::LzmaOptions::get_memory_usage`
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
lzma-rust2 = { git = "https://github.com/hasenbanck/lzma-rust2", rev = "d6268a287ceb288114a02a5f29da28acaaaf9e52" }
```

`src/main.rs`:

```rust
use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;

fn main() {
    let lzma_options = LzmaOptions { dict_size: u32::MAX, lc: 0u32, lp: 0u32, pb: 0u32, mode: EncodeMode::Fast, nice_len: 0u32, mf: MfType::Hc4, depth_limit: 0i32, preset_dict: None };
    let _ = lzma_options.get_memory_usage();
}
```

Run with `cargo run` in this directory (package `repro_lzma_rust2_src_lz_lz_encoder_rs_467_28_99396a`).

## Output

```
thread 'main' (460453) panicked at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/lz_encoder.rs:467:28:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: lzma_rust2::lz::lz_encoder::get_buf_size
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/lz_encoder.rs:467:28
   4: <lzma_rust2::lz::lz_encoder::LzEncoder>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/lz_encoder.rs:102:9
   5: <lzma_rust2::enc::encoder_fast::FastEncoderMode>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/encoder_fast.rs:15:9
   6: <lzma_rust2::enc::encoder::LzmaEncoder>::get_mem_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/encoder.rs:126:33
   7: <lzma_rust2::enc::lzma2_writer::LzmaOptions>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/lzma2_writer.rs:148:9
   8: repro_lzma_rust2_src_lz_lz_encoder_rs_467_28_99396a::main
             at ./src/main.rs:7:26
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
