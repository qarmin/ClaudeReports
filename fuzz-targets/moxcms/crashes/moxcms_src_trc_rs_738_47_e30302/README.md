# moxcms/src/trc.rs:738:47

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/trc.rs:738:47`
- last API call: `moxcms::ToneReprCurve::inverse`
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
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f" }
```

`src/main.rs`:

```rust
use moxcms::ToneReprCurve;

fn main() {
    let _ = ToneReprCurve::Lut(vec![]).inverse();
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_trc_rs_738_47_e30302`).

## Output

```
thread 'main' (356545) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/trc.rs:738:47:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: moxcms::trc::lut_interp_linear16
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/trc.rs:738:47
   4: moxcms::trc::lut_inverse_interp16
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/trc.rs:889:15
   5: moxcms::trc::invert_lut
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/trc.rs:940:16
   6: <moxcms::trc::ToneReprCurve>::inverse
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/trc.rs:48:39
   7: repro_moxcms_src_trc_rs_738_47_e30302::main
             at ./src/main.rs:4:40
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
