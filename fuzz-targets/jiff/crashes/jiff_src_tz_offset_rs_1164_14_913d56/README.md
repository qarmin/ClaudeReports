# jiff/src/tz/offset.rs:1164:14

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/tz/offset.rs:1164:14`
- last API call: `jiff::tz::Offset::add<jiff::Span>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding span to offset should not overflow: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::SignedDuration;
use jiff::Span;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::tz::Offset as ::core::ops::Add<jiff::Span>>::add(Offset::from_seconds(0i32).unwrap(), Span::try_from(SignedDuration::from_hours(91i64)).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_tz_offset_rs_1164_14_913d56`).

## Output

```
thread 'main' (361513) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/tz/offset.rs:1164:14:
adding span to offset should not overflow: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::tz::offset::Offset, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::tz::offset::Offset as core::ops::arith::Add<jiff::span::Span>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/tz/offset.rs:1164:14
   5: repro_jiff_src_tz_offset_rs_1164_14_913d56::main
             at ./src/main.rs:6:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
