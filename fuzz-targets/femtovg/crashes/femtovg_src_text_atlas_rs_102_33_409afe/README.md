# femtovg/src/text/atlas.rs:102:33

- crate: femtovg 0.27.0
- kind: panic
- location: `femtovg/src/text/atlas.rs:102:33`
- source: https://docs.rs/crate/femtovg/0.27.0/source/src/text/atlas.rs#102
- last API call: `femtovg::Atlas::add_rect`
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
femtovg = { version = "=0.27.0" }
```

`src/main.rs`:

```rust
use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(1000000000000000000usize, 0usize);
    atlas.expand(10000000000000000000usize, 1usize);
    let _ = atlas.add_rect(10000000000000000000usize, 1usize);
}
```

Run with `cargo run` in this directory (package `repro_femtovg_src_text_atlas_rs_102_33_409afe`).

## Output

```
thread 'main' (251622) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:102:33:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <femtovg::text::atlas::Atlas>::add_skyline_level
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:102:33
   4: <femtovg::text::atlas::Atlas>::add_rect
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:75:18
   5: repro_femtovg_src_text_atlas_rs_102_33_409afe::main
             at ./src/main.rs:6:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
