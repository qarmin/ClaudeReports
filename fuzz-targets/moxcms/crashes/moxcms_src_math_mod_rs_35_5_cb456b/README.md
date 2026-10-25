# moxcms/src/math/mod.rs:35:5

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/math/mod.rs:35:5`
- last API call: `moxcms::rounding_div_ceil`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to divide by zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = moxcms::rounding_div_ceil(0i32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_math_mod_rs_35_5_cb456b`).

## Output

```
thread 'main' (356508) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/math/mod.rs:35:5:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: moxcms::math::rounding_div_ceil
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/math/mod.rs:35:5
   4: repro_moxcms_src_math_mod_rs_35_5_cb456b::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
