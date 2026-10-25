# vello_common/src/strip.rs:255:9

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/strip.rs:255:9`
- last API call: `vello_common::strip::Strip::set_alpha_idx`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15735399285831242918

## Message

```
`alpha_idx` too large
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::strip::Strip;

fn main() {
    let mut strip = Strip::sentinel(0u16, 0u32);
    strip.set_alpha_idx(u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_strip_rs_255_9_fbf708`).

## Output

```
thread 'main' (1167064) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:255:9:
`alpha_idx` too large
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::strip::Strip>::set_alpha_idx
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:255:9
   3: repro_vello_common_src_strip_rs_255_9_fbf708::main
             at ./src/main.rs:5:11
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
