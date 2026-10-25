# float-cmp/src/ulps_eq.rs:91:17

- crate: https://github.com/RazrFalcon/strict-num @ 76d36febdb73e875ae3e0c6ae1310a084a7860c7
- kind: panic
- location: `float-cmp/src/ulps_eq.rs:91:17`
- source: https://docs.rs/crate/float-cmp/0.9.0/source/src/ulps_eq.rs#91
- last API call: `strict_num::PositiveF64::approx_eq_ulps`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to negate with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
strict-num = { git = "https://github.com/RazrFalcon/strict-num", rev = "76d36febdb73e875ae3e0c6ae1310a084a7860c7" }
```

`src/main.rs`:

```rust
use strict_num::PositiveF64;

fn main() {
    let _ = <strict_num::PositiveF64 as strict_num::ApproxEqUlps>::approx_eq_ulps(&PositiveF64::new(1.0f64).unwrap(), &PositiveF64::ZERO, i64::MIN);
}
```

Run with `cargo run` in this directory (package `repro_float_cmp_src_ulps_eq_rs_91_17_5b7a28`).

## Output

```
thread 'main' (545912) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/float-cmp-0.9.0/src/ulps_eq.rs:91:17:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <f64 as float_cmp::ulps_eq::ApproxEqUlps>::approx_eq_ulps
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/float-cmp-0.9.0/src/ulps_eq.rs:91:17
   4: <strict_num::FiniteF64 as float_cmp::ulps_eq::ApproxEqUlps>::approx_eq_ulps
             at /home/rafal/.cargo/git/checkouts/strict-num-0568e24592aed8c8/76d36fe/src/lib.rs:93:24
   5: <strict_num::PositiveF64 as float_cmp::ulps_eq::ApproxEqUlps>::approx_eq_ulps
             at /home/rafal/.cargo/git/checkouts/strict-num-0568e24592aed8c8/76d36fe/src/lib.rs:93:24
   6: repro_float_cmp_src_ulps_eq_rs_91_17_5b7a28::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
