# lofty/src/picture/mod.rs:479:53

- crate: https://github.com/Serial-ATA/lofty-rs @ f88b02cfa787debffff99c341b54c3a77b83b832
- kind: panic
- location: `lofty/src/picture/mod.rs:479:53`
- last API call: `lofty::picture::PictureInformation::from_jpeg`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lofty = { git = "https://github.com/Serial-ATA/lofty-rs", rev = "f88b02cfa787debffff99c341b54c3a77b83b832" }
```

`src/main.rs`:

```rust
use lofty::picture::PictureInformation;

fn main() {
    let _ = PictureInformation::from_jpeg(&vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8]);
}
```

Run with `cargo run` in this directory (package `repro_lofty_src_picture_mod_rs_479_53_7edac3`).

## Output

```
thread 'main' (3011033) panicked at /home/rafal/.cargo/git/checkouts/lofty-rs-fdd1bd466a957cbb/f88b02c/lofty/src/picture/mod.rs:479:53:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <lofty::picture::PictureInformation>::from_jpeg
             at /home/rafal/.cargo/git/checkouts/lofty-rs-fdd1bd466a957cbb/f88b02c/lofty/src/picture/mod.rs:479:44
   4: repro_lofty_src_picture_mod_rs_479_53_7edac3::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
