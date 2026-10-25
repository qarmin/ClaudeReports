# taffy/src/compute/common/alignment.rs:49:5

- crate: https://github.com/DioxusLabs/taffy @ 8bb96a26f91353e951b373b4db2c716e37e70c20
- kind: panic
- location: `taffy/src/compute/common/alignment.rs:49:5`
- last API call: `taffy::compute_block_align_content_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
normal must be resolved before computing content alignment
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
taffy = { git = "https://github.com/DioxusLabs/taffy", rev = "8bb96a26f91353e951b373b4db2c716e37e70c20" }
```

`src/main.rs`:

```rust
use taffy::prelude::AlignContent;

fn main() {
    let _ = taffy::compute_block_align_content_offset(AlignContent::default(), 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_taffy_src_compute_common_alignment_rs_49_5_38f2fe`).

## Output

```
thread 'main' (1356340) panicked at /home/rafal/.cargo/git/checkouts/taffy-bd4586f5551dc460/8bb96a2/src/compute/common/alignment.rs:49:5:
normal must be resolved before computing content alignment
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: taffy::compute::common::alignment::apply_alignment_fallback
             at /home/rafal/.cargo/git/checkouts/taffy-bd4586f5551dc460/8bb96a2/src/compute/common/alignment.rs:49:5
   3: taffy::compute::block::compute_block_align_content_offset
             at /home/rafal/.cargo/git/checkouts/taffy-bd4586f5551dc460/8bb96a2/src/compute/block.rs:36:19
   4: repro_taffy_src_compute_common_alignment_rs_49_5_38f2fe::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
