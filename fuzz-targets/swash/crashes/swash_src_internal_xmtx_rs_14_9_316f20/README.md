# swash/src/internal/xmtx.rs:14:9

- crate: https://github.com/dfrg/swash @ 7773843df0d63cd468db61a29c152b5e7a99d4ab
- kind: panic
- location: `swash/src/internal/xmtx.rs:14:9`
- last API call: `swash::GlyphMetrics::advance_width`
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
swash = { git = "https://github.com/dfrg/swash", rev = "7773843df0d63cd468db61a29c152b5e7a99d4ab" }
```

`src/main.rs`:

```rust
use swash::FontRef;

fn main() {
    let vec = vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8];
    let font_ref = FontRef::from_offset(&vec, 16u32).unwrap();
    let vec2 = vec![];
    let glyph_metrics = font_ref.glyph_metrics(&vec2);
    let _ = glyph_metrics.advance_width(0u16);
}
```

Run with `cargo run` in this directory (package `repro_swash_src_internal_xmtx_rs_14_9_316f20`).

## Output

```
thread 'main' (571263) panicked at /home/rafal/.cargo/git/checkouts/swash-49931c79b833dc98/7773843/src/internal/xmtx.rs:14:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: swash::internal::xmtx::advance
             at /home/rafal/.cargo/git/checkouts/swash-49931c79b833dc98/7773843/src/internal/xmtx.rs:14:9
   4: <swash::metrics::GlyphMetrics>::advance_width
             at /home/rafal/.cargo/git/checkouts/swash-49931c79b833dc98/7773843/src/metrics.rs:381:21
   5: repro_swash_src_internal_xmtx_rs_14_9_316f20::main
             at ./src/main.rs:8:27
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
