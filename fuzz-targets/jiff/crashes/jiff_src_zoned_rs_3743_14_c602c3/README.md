# jiff/src/zoned.rs:3743:14

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/zoned.rs:3743:14`
- last API call: `jiff::Zoned::add<jiff::SignedDuration>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding signed duration to zoned datetime overflowed: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
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
use jiff::Zoned;

fn main() {
    let _ = <jiff::Zoned as ::core::ops::Add<jiff::SignedDuration>>::add(Zoned::default(), SignedDuration::from_secs(-1000000000000000000i64));
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_zoned_rs_3743_14_c602c3`).

## Output

```
thread 'main' (361787) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/zoned.rs:3743:14:
adding signed duration to zoned datetime overflowed: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::zoned::Zoned, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::zoned::Zoned as core::ops::arith::Add<jiff::signed_duration::SignedDuration>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/zoned.rs:3743:14
   5: repro_jiff_src_zoned_rs_3743_14_c602c3::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
