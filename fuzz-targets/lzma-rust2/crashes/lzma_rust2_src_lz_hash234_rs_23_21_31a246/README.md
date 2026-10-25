# lzma-rust2/src/lz/hash234.rs:23:21

- crate: https://github.com/hasenbanck/lzma-rust2 @ d6268a287ceb288114a02a5f29da28acaaaf9e52
- kind: panic
- location: `lzma-rust2/src/lz/hash234.rs:23:21`
- last API call: `lzma_rust2::LzmaOptions::get_memory_usage`
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
lzma-rust2 = { git = "https://github.com/hasenbanck/lzma-rust2", rev = "d6268a287ceb288114a02a5f29da28acaaaf9e52" }
```

`src/main.rs`:

```rust
use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;
use lzma_rust2::filter::bcj2::Bcj2Options;

fn main() {
    let u32_v = lzma_rust2::get_extra_size_before(1000000000u32);
    let u32_v2 = Bcj2Options::RELATIVE_LIMIT_DEFAULT;
    let lzma_options = LzmaOptions::new(u32_v, 0u32, 0u32, u32_v2, EncodeMode::Normal, 0u32, MfType::Hc4, 0i32);
    let _ = lzma_options.get_memory_usage();
}
```

Run with `cargo run` in this directory (package `repro_lzma_rust2_src_lz_hash234_rs_23_21_31a246`).

## Output

```
thread 'main' (460415) panicked at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/hash234.rs:23:21:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <lzma_rust2::lz::hash234::Hash234>::get_hash4_size
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/hash234.rs:23:21
   4: <lzma_rust2::lz::hash234::Hash234>::get_mem_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/hash234.rs:37:36
   5: <lzma_rust2::lz::hc4::Hc4>::get_mem_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/hc4.rs:21:9
   6: <lzma_rust2::lz::lz_encoder::MfType>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/lz_encoder.rs:51:28
   7: <lzma_rust2::lz::lz_encoder::LzEncoder>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/lz/lz_encoder.rs:109:18
   8: <lzma_rust2::enc::encoder_normal::NormalEncoderMode>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/encoder_normal.rs:24:9
   9: <lzma_rust2::enc::encoder::LzmaEncoder>::get_mem_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/encoder.rs:128:17
  10: <lzma_rust2::enc::lzma2_writer::LzmaOptions>::get_memory_usage
             at /home/rafal/.cargo/git/checkouts/lzma-rust2-a2492db1605189c0/d6268a2/src/enc/lzma2_writer.rs:148:9
  11: repro_lzma_rust2_src_lz_hash234_rs_23_21_31a246::main
             at ./src/main.rs:10:26
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
