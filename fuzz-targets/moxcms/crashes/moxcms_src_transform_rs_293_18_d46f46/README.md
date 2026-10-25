# moxcms/src/transform.rs:293:18

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/transform.rs:293:18`
- last API call: `moxcms::Layout::from<u8>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f" }
```

`src/main.rs`:

```rust
use moxcms::Layout;

fn main() {
    let _ = Layout::from(10u8);
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_transform_rs_293_18_d46f46`).

## Output

```
thread 'main' (356415) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/transform.rs:293:18:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <moxcms::transform::Layout as core::convert::From<u8>>::from
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/transform.rs:293:18
   4: repro_moxcms_src_transform_rs_293_18_d46f46::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
