# femtovg/src/text/atlas.rs:160:13

- crate: femtovg 0.27.0
- kind: panic
- location: `femtovg/src/text/atlas.rs:160:13`
- source: https://docs.rs/crate/femtovg/0.27.0/source/src/text/atlas.rs#160
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
    let mut atlas = Atlas::new(10000000000000000000usize, 0usize);
    let _ = atlas.add_rect(1000000000000000000usize, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_femtovg_src_text_atlas_rs_160_13_f0d024`).

## Output

```
thread 'main' (232596) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:160:13:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <femtovg::text::atlas::Atlas>::rect_fits
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:160:13
   4: <femtovg::text::atlas::Atlas>::add_rect
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/text/atlas.rs:62:35
   5: repro_femtovg_src_text_atlas_rs_160_13_f0d024::main
             at ./src/main.rs:5:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
