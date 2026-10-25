# vello_common/src/flatten_simd.rs:94:9

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/flatten_simd.rs:94:9`
- last API call: `vello_common::clip::ClipContext::push_clip_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1122092412460416190

## Message

```
Non-empty paths must begin with `PathEl::MoveTo`, got ClosePath
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
fearless_simd = { version = "=1.1.0" }
peniko = { version = "=0.6.1" }
```

`src/main.rs`:

```rust
use fearless_simd::Level;
use kurbo::Affine;
use kurbo::PathEl;
use peniko::Fill;
use vello_common::clip::ClipContext;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_context = ClipContext::default();
    let mut strip_generator = StripGenerator::new(0u16, 0u16, Level::new());
    clip_context.push_clip_path(vec![PathEl::ClosePath], &mut strip_generator, Fill::NonZero, Affine::scale_non_uniform(0.0f64, 0.0f64), None);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_flatten_simd_rs_94_9_f81a82`).

## Output

```
thread 'main' (1211069) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/flatten_simd.rs:94:9:
Non-empty paths must begin with `PathEl::MoveTo`, got ClosePath
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: vello_common::flatten_simd::flatten::<fearless_simd::generated::avx512::Avx512, alloc::vec::Vec<kurbo::bezpath::PathEl>, vello_common::flatten::FlattenerCallback>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/flatten_simd.rs:94:9
   3: vello_common::flatten::fill_impl::<fearless_simd::generated::avx512::Avx512, alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/flatten.rs:209:5
   4: vello_common::flatten::fill::<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#3}
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/flatten.rs:186:30
   5: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::vectorize_avx512::<vello_common::flatten::fill<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#3}, ()>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-1.1.0/src/generated/avx512.rs:396:13
   6: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::<vello_common::flatten::fill<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#3}, ()>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-1.1.0/src/generated/avx512.rs:398:18
   7: vello_common::flatten::fill::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/flatten.rs:186:5
   8: <vello_common::strip_generator::StripGenerator>::generate_filled_path::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip_generator.rs:132:9
   9: <vello_common::clip::ClipContext>::push_clip_path::<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:101:23
  10: <vello_common::clip::ClipContext>::push_generated_clip::<<vello_common::clip::ClipContext>::push_clip_path<alloc::vec::Vec<kurbo::bezpath::PathEl>>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:133:21
  11: <vello_common::clip::ClipContext>::push_clip_path::<alloc::vec::Vec<kurbo::bezpath::PathEl>>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:100:14
  12: repro_vello_common_src_flatten_simd_rs_94_9_f81a82::main
             at ./src/main.rs:11:18
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
