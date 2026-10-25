# vello_common/src/util.rs:430:52

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/util.rs:430:52`
- last API call: `vello_common::clip::ClipContext::push_clip_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12097560963160117118

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
fearless_simd = { version = "=1.1.0" }
```

`src/main.rs`:

```rust
use fearless_simd::Level;
use kurbo::Rect;
use vello_common::clip::ClipContext;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_context = ClipContext::default();
    let mut strip_generator = StripGenerator::new(1u16, u16::MAX, Level::baseline());
    clip_context.push_clip_rect(&Rect { x0: 1.0f64, y0: 960000.0f64, x1: 0.0f64, y1: 0.0f64 }, &mut strip_generator);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_util_rs_430_52_5fba5b`).

## Output

```
thread 'main' (1183363) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:430:52:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<u16>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: vello_common::util::strip_bbox
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:430:52
   6: <vello_common::clip::ClipContext>::push_generated_clip::<<vello_common::clip::ClipContext>::push_clip_rect::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:135:20
   7: <vello_common::clip::ClipContext>::push_clip_rect
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:116:14
   8: repro_vello_common_src_util_rs_430_52_5fba5b::main
             at ./src/main.rs:9:18
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
