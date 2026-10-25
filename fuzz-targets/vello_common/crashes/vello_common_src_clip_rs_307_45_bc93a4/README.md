# vello_common/src/clip.rs:307:45

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/clip.rs:307:45`
- last API call: `vello_common::viewport::ViewportState::pop_clip`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13743704618384564857

## Message

```
clip stack underflowed
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
fearless_simd = { version = "=1.1.0" }
```

`src/main.rs`:

```rust
use fearless_simd::Level;
use vello_common::viewport::ViewportState;

fn main() {
    let mut viewport_state = ViewportState::new(0u16, 0u16, Level::baseline());
    viewport_state.pop_clip();
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_clip_rs_307_45_bc93a4`).

## Output

```
thread 'main' (1143367) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:307:45:
clip stack underflowed
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<vello_common::clip::RawClip>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <vello_common::clip::ClipState>::pop_clip
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:307:45
   6: <vello_common::viewport::ViewportState>::pop_clip
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/viewport.rs:90:25
   7: repro_vello_common_src_clip_rs_307_45_bc93a4::main
             at ./src/main.rs:6:20
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
