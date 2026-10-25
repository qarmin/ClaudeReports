# vello_common/src/geometry.rs:57:36

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/geometry.rs:57:36`
- last API call: `vello_common::geometry::SizeU16::clamp`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12323504626823885750

## Message

```
min > max. min = 1, max = 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::geometry::SizeU16;

fn main() {
    let size_u16 = SizeU16::from((0u16, 0u16));
    let _ = size_u16.clamp(1u16, 0u16);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_geometry_rs_57_36_2c0aa2`).

## Output

```
thread 'main' (1146836) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/geometry.rs:57:36:
min > max. min = 1, max = 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <u16 as core::cmp::Ord>::clamp::do_panic::runtime
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:172:21
   3: <u16 as core::cmp::Ord>::clamp::do_panic
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:2636:9
   4: <u16 as core::cmp::Ord>::clamp
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:177:9
   5: <vello_common::geometry::SizeU16>::clamp
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/geometry.rs:57:36
   6: repro_vello_common_src_geometry_rs_57_36_2c0aa2::main
             at ./src/main.rs:5:22
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
