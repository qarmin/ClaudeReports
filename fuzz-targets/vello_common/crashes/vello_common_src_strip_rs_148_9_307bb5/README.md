# vello_common/src/strip.rs:148:9

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/strip.rs:148:9`
- last API call: `vello_common::util::strip_bbox`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4553298341562749030

## Message

```
assertion `left == right` failed: strip width must be tile-width aligned
  left: 1
 right: 0
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
    let _ = vello_common::util::strip_bbox(&[Strip::new(0u16, 0u16, 0u32, false), Strip::new(0u16, 0u16, 100u32, false)]);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_strip_rs_148_9_307bb5`).

## Output

```
thread 'main' (1182720) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:148:9:
assertion `left == right` failed: strip width must be tile-width aligned
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<u16, u16>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: vello_common::strip::visit_strip_fill_segments::<vello_common::geometry::RectU16, vello_common::util::strip_bbox::{closure#0}, vello_common::util::strip_bbox::{closure#1}>
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:148:9
   5: vello_common::util::strip_bbox
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/util.rs:409:5
   6: repro_vello_common_src_strip_rs_148_9_307bb5::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
