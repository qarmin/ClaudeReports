# contract: PartialOrdEq for moxcms::LCh

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `moxcms::LCh::div<f32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: PartialOrdEq for moxcms::LCh
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f" }
```

`src/main.rs`:

```rust
use moxcms::LCh;

fn main() {
    let l_ch = <moxcms::LCh as ::core::ops::Div<f32>>::div(<moxcms::LCh as ::core::ops::Add<f32>>::add(LCh::default(), 1.0f32), 0.0f32);
    assert_eq!(l_ch == l_ch, l_ch.partial_cmp(&l_ch) == Some(std::cmp::Ordering::Equal), "PartialOrd inconsistent with PartialEq");
}
```

Run with `cargo run` in this directory (package `repro_contract_partialordeq_for_moxcms_lch_af2ca2`).

## Output

```
thread 'main' (356590) panicked at src/main.rs:5:5:
assertion `left == right` failed: PartialOrd inconsistent with PartialEq
  left: false
 right: true
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<bool, bool>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: repro_contract_partialordeq_for_moxcms_lch_af2ca2::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
