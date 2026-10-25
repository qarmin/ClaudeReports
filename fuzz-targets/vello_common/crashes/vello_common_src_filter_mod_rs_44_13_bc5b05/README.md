# vello_common/src/filter/mod.rs:44:13

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/filter/mod.rs:44:13`
- last API call: `vello_common::filter::PreparedFilter::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12105399757288728460

## Message

```
not implemented: Multi-primitive filter graphs are not yet supported
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterGraph;
use vello_common::transforms::RootTransforms;
use vello_common::transforms::Transforms;

fn main() {
    let root_transforms = RootTransforms::new();
    let affine = root_transforms.effective_path_transform(&Transforms::default());
    let _ = PreparedFilter::new(&Filter { graph: std::sync::Arc::new(FilterGraph::new()) }, &affine);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_filter_mod_rs_44_13_bc5b05`).

## Output

```
thread 'main' (1145402) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:44:13:
not implemented: Multi-primitive filter graphs are not yet supported
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::filter::PreparedFilter>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/mod.rs:44:13
   3: repro_vello_common_src_filter_mod_rs_44_13_bc5b05::main
             at ./src/main.rs:10:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
