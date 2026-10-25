# lzma-rs/src/decode/lzma.rs:65:9

- crate: https://github.com/gendx/lzma-rs @ 1f14478def43b8ea0d27a6b3db88bb307aafd18a
- kind: panic
- location: `lzma-rs/src/decode/lzma.rs:65:9`
- last API call: `lzma_rs::decompress::raw::LzmaDecoder::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.pb <= 4
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lzma-rs = { git = "https://github.com/gendx/lzma-rs", rev = "1f14478def43b8ea0d27a6b3db88bb307aafd18a", features = ["enable_logging", "env_logger", "log", "raw_decoder", "stream"] }
```

`src/main.rs`:

```rust
use lzma_rs::decompress::raw::LzmaDecoder;
use lzma_rs::decompress::raw::LzmaParams;
use lzma_rs::decompress::raw::LzmaProperties;

fn main() {
    let _ = LzmaDecoder::new(LzmaParams::new(LzmaProperties { lc: 0u32, lp: 1u32, pb: 100u32 }, 0u32, None), None);
}
```

Run with `cargo run` in this directory (package `repro_lzma_rs_src_decode_lzma_rs_65_9_141ec2`).

## Output

```
thread 'main' (1409383) panicked at /home/rafal/.cargo/git/checkouts/lzma-rs-7a9a77f98e2661bf/1f14478/src/decode/lzma.rs:65:9:
assertion failed: self.pb <= 4
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <lzma_rs::decode::lzma::LzmaProperties>::validate
             at /home/rafal/.cargo/git/checkouts/lzma-rs-7a9a77f98e2661bf/1f14478/src/decode/lzma.rs:65:9
   4: <lzma_rs::decode::lzma::DecoderState>::new
             at /home/rafal/.cargo/git/checkouts/lzma-rs-7a9a77f98e2661bf/1f14478/src/decode/lzma.rs:189:20
   5: <lzma_rs::decode::lzma::LzmaDecoder>::new
             at /home/rafal/.cargo/git/checkouts/lzma-rs-7a9a77f98e2661bf/1f14478/src/decode/lzma.rs:611:20
   6: repro_lzma_rs_src_decode_lzma_rs_65_9_141ec2::main
             at ./src/main.rs:6:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
