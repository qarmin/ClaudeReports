# vello_common/src/filter_effects.rs:84:18

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/filter_effects.rs:84:18`
- last API call: `vello_common::filter_effects::Filter::from_function`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9109968461502764535

## Message

```
not implemented: Filter function Contrast { amount: 0.0 } not supported
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterFunction;

fn main() {
    let _ = Filter::from_function(FilterFunction::Contrast { amount: 0.0f32 });
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_filter_effects_rs_84_18_3c4f3a`).

## Output

```
thread 'main' (1146415) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter_effects.rs:84:18:
not implemented: Filter function Contrast { amount: 0.0 } not supported
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::filter_effects::Filter>::from_function
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter_effects.rs:84:18
   3: repro_vello_common_src_filter_effects_rs_84_18_3c4f3a::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
