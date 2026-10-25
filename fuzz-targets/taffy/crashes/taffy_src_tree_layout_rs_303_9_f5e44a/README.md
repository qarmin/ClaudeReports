# taffy/src/tree/layout.rs:303:9

- crate: https://github.com/DioxusLabs/taffy @ 8bb96a26f91353e951b373b4db2c716e37e70c20
- kind: panic
- location: `taffy/src/tree/layout.rs:303:9`
- last API call: `taffy::tree::AxisStaticPosition::from_alignment`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
self-start/self-end must be resolved before computing the static position
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
taffy = { git = "https://github.com/DioxusLabs/taffy", rev = "8bb96a26f91353e951b373b4db2c716e37e70c20" }
```

`src/main.rs`:

```rust
use taffy::prelude::AlignItems;
use taffy::tree::AxisStaticPosition;

fn main() {
    let _ = AxisStaticPosition::from_alignment(AlignItems::SAFE_SELF_START, <taffy::geometry::Line<f32> as taffy::prelude::FromLength>::from_length::<f64>(1.0f64), false);
}
```

Run with `cargo run` in this directory (package `repro_taffy_src_tree_layout_rs_303_9_f5e44a`).

## Output

```
thread 'main' (488225) panicked at /home/rafal/.cargo/git/checkouts/taffy-bd4586f5551dc460/8bb96a2/src/tree/layout.rs:303:9:
self-start/self-end must be resolved before computing the static position
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <taffy::tree::layout::AxisStaticPosition>::from_alignment
             at /home/rafal/.cargo/git/checkouts/taffy-bd4586f5551dc460/8bb96a2/src/tree/layout.rs:303:9
   3: repro_taffy_src_tree_layout_rs_303_9_f5e44a::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
