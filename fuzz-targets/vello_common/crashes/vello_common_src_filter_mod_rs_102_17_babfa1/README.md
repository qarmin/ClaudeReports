# vello_common/src/filter/mod.rs:102:17

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/filter/mod.rs:102:17`
- last API call: `vello_common::filter::PreparedFilter::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16650349921081099109

## Message

```
not implemented: Other filter primitives not yet implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
```

`src/main.rs`:

```rust
use kurbo::Affine;
use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = PreparedFilter::new(&Filter::from_primitive(FilterPrimitive::Image { image_id: 0u32, transform: None }), &Affine::scale(0.0f64));
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_filter_mod_rs_102_17_babfa1`).

## Output

```
thread 'main' (1156075) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:102:17:
not implemented: Other filter primitives not yet implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::filter::PreparedFilter>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:102:17
   3: repro_vello_common_src_filter_mod_rs_102_17_babfa1::main
             at ./src/main.rs:7:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
