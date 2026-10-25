# vello_common/src/geometry.rs:93:51

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/geometry.rs:93:51`
- last API call: `vello_common::geometry::SizeU16::add<vello_common::geometry::SizeU16>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18257912824699227857

## Message

```
called `Option::unwrap()` on a `None` value
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
    let _ = <vello_common::geometry::SizeU16 as ::core::ops::Add<vello_common::geometry::SizeU16>>::add(SizeU16([u16::MAX, 0u16]), SizeU16::new(1u16));
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_geometry_rs_93_51_0e7a54`).

## Output

```
thread 'main' (1154990) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/geometry.rs:93:51:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<u16>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <vello_common::geometry::SizeU16 as core::ops::arith::Add>::add
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/geometry.rs:93:51
   6: repro_vello_common_src_geometry_rs_93_51_0e7a54::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
