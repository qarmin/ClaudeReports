# vello_common/src/viewport.rs:120:14

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/viewport.rs:120:14`
- last API call: `vello_common::viewport::ViewportState::pop_root_viewport`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7252105841824247062

## Message

```
root viewport stack underflow
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
    viewport_state.pop_root_viewport();
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_viewport_rs_120_14_a72b21`).

## Output

```
thread 'main' (1155659) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/viewport.rs:120:14:
root viewport stack underflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<vello_common::strip_generator::StripGenerator>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <vello_common::viewport::ViewportState>::pop_root_viewport
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/viewport.rs:120:14
   6: repro_vello_common_src_viewport_rs_120_14_a72b21::main
             at ./src/main.rs:6:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
