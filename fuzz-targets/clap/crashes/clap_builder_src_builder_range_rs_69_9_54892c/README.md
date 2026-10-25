# clap_builder/src/builder/range.rs:69:9

- crate: https://github.com/clap-rs/clap @ ccc89b83875474399e9d9d70841163f6319d1a41
- kind: panic
- location: `clap_builder/src/builder/range.rs:69:9`
- last API call: `clap_builder::builder::ValueRange::from<::core::ops::Range<usize>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: start_inclusive <= end_inclusive
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
clap = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
clap_builder = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
```

`src/main.rs`:

```rust
use clap_builder::builder::ValueRange;

fn main() {
    let _ = ValueRange::from(usize::MAX..12usize);
}
```

Run with `cargo run` in this directory (package `repro_clap_builder_src_builder_range_rs_69_9_54892c`).

## Output

```
thread 'main' (458946) panicked at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_builder/src/builder/range.rs:69:9:
assertion failed: start_inclusive <= end_inclusive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <clap_builder::builder::range::ValueRange>::raw
             at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_builder/src/builder/range.rs:69:9
   4: <clap_builder::builder::range::ValueRange as core::convert::From<core::ops::range::Range<usize>>>::from
             at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_builder/src/builder/range.rs:150:9
   5: repro_clap_builder_src_builder_range_rs_69_9_54892c::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
