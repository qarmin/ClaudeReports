# vello_common/src/clip.rs:253:39

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/clip.rs:253:39`
- last API call: `vello_common::clip::ClipState::pop_root_viewport`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 10999149196703537178

## Message

```
filter clip stack underflow
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
use vello_common::clip::ClipState;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_state = ClipState::default();
    let mut strip_generator = StripGenerator::new(0u16, 0u16, Level::try_detect().unwrap());
    clip_state.pop_root_viewport(&mut strip_generator);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_clip_rs_253_39_eb067c`).

## Output

```
thread 'main' (1165085) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:253:39:
filter clip stack underflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<vello_common::clip::ClipFrame>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <vello_common::clip::ClipState>::pop_root_viewport
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/clip.rs:253:39
   6: repro_vello_common_src_clip_rs_253_39_eb067c::main
             at ./src/main.rs:8:16
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
