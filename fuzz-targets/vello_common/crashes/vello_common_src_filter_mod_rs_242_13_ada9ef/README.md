# vello_common/src/filter/mod.rs:242:13

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/filter/mod.rs:242:13`
- last API call: `vello_common::filter::FilterData::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13441334375059198398

## Message

```
filter expansion must contain the origin
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
use vello_common::filter::FilterData;
use vello_common::filter_effects::CompositeOperator;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = FilterData::new(Filter::from_primitive(FilterPrimitive::Composite { operator: CompositeOperator::In }), Affine::new([0.0f64, 0.0f64, f64::NEG_INFINITY, 0.0f64, 0.0f64, 0.0f64]));
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_filter_mod_rs_242_13_ada9ef`).

## Output

```
thread 'main' (1166714) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:242:13:
filter expansion must contain the origin
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::filter::FilterData>::new::snapped_padding
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:242:13
   3: <vello_common::filter::FilterData>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:264:30
   4: repro_vello_common_src_filter_mod_rs_242_13_ada9ef::main
             at ./src/main.rs:8:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
