# pxfm/src/logs/log1p_dd.rs:241:40

- crate: https://github.com/awxkee/pxfm @ acf7acd00cfc845307eda7b7b2de19e42303de82
- kind: panic
- location: `pxfm/src/logs/log1p_dd.rs:241:40`
- last API call: `pxfm::f_betainc_reg`
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
pxfm = { git = "https://github.com/awxkee/pxfm", rev = "acf7acd00cfc845307eda7b7b2de19e42303de82" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = pxfm::f_betainc_reg(1.0f64, 1e300f64, 4.9980524806478024e-203f64);
}
```

Run with `cargo run` in this directory (package `repro_pxfm_src_logs_log1p_dd_rs_241_40_8839b2`).

## Output

```
thread 'main' (577454) panicked at /home/rafal/.cargo/git/checkouts/pxfm-7a4f3bbae50a10e6/acf7acd/src/logs/log1p_dd.rs:241:40:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: pxfm::logs::log1p_dd::log1p_fast_dd
             at /home/rafal/.cargo/git/checkouts/pxfm-7a4f3bbae50a10e6/acf7acd/src/logs/log1p_dd.rs:241:40
   4: pxfm::gamma::betainc::f_betainc_reg
             at /home/rafal/.cargo/git/checkouts/pxfm-7a4f3bbae50a10e6/acf7acd/src/gamma/betainc.rs:105:24
   5: repro_pxfm_src_logs_log1p_dd_rs_241_40_8839b2::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
