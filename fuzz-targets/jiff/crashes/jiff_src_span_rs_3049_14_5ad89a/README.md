# jiff/src/span.rs:3049:14

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/span.rs:3049:14`
- last API call: `jiff::Span::mul<i64>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
multiplying `Span` by a scalar overflowed: parameter 'minutes' is not in the required range of -10518456960..=10518456960
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::Span as ::core::ops::Mul<i64>>::mul(<jiff::tz::Offset as ::core::ops::Sub<jiff::tz::Offset>>::sub(Offset::UTC, Offset::from_seconds(0i32).unwrap()).minutes::<i64>(1i64), -1000000000000000000i64);
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_span_rs_3049_14_5ad89a`).

## Output

```
thread 'main' (361234) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/span.rs:3049:14:
multiplying `Span` by a scalar overflowed: parameter 'minutes' is not in the required range of -10518456960..=10518456960
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::span::Span, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::span::Span as core::ops::arith::Mul<i64>>::mul
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/span.rs:3049:14
   5: repro_jiff_src_span_rs_3049_14_5ad89a::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
