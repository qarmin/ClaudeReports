# vello_common/src/util.rs:451:13

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/util.rs:451:13`
- last API call: `vello_common::pixmap::unpremultiply_rgba8`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 10520332042853067809

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = vello_common::pixmap::unpremultiply_rgba8(&mut vec![102u8, 179u8, 217u8, 108u8]);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_util_rs_451_13_cd178d`).

## Output

```
thread 'main' (1220491) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:451:13:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u16 as core::ops::arith::Add>::add
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:104:45
   4: <u16 as vello_common::util::unpremultiply::Div256Ext>::div_256
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:451:13
   5: vello_common::util::unpremultiply::scalar
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:495:55
   6: vello_common::pixmap::unpremultiply_rgba8_impl::<fearless_simd::generated::avx512::Avx512>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/pixmap.rs:589:26
   7: vello_common::pixmap::unpremultiply_rgba8::{closure#3}
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/pixmap.rs:565:30
   8: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::vectorize_avx512::<vello_common::pixmap::unpremultiply_rgba8::{closure#3}, bool>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-1.1.0/src/generated/avx512.rs:396:13
   9: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::<vello_common::pixmap::unpremultiply_rgba8::{closure#3}, bool>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-1.1.0/src/generated/avx512.rs:398:18
  10: vello_common::pixmap::unpremultiply_rgba8
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/pixmap.rs:565:5
  11: repro_vello_common_src_util_rs_451_13_cd178d::main
             at ./src/main.rs:2:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
