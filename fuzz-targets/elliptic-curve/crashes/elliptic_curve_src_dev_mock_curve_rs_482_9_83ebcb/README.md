# elliptic-curve/src/dev/mock_curve.rs:482:9

- crate: elliptic-curve 0.14.1
- kind: panic
- location: `elliptic-curve/src/dev/mock_curve.rs:482:9`
- source: https://docs.rs/crate/elliptic-curve/0.14.1/source/src/dev/mock_curve.rs#482
- last API call: `elliptic_curve::dev::mock_curve::AffinePoint::to_curve`
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
elliptic-curve = { version = "=0.14.1", features = ["alloc", "arithmetic", "dev", "digest", "ecdh", "ff", "getrandom", "group", "pem", "pkcs8", "sec1", "serde", "std"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::to_curve(&<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::generator());
}
```

Run with `cargo run` in this directory (package `repro_elliptic_curve_src_dev_mock_curve_rs_482_9_83ebcb`).

## Output

```
thread 'main' (2582203) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/elliptic-curve-0.14.1/src/dev/mock_curve.rs:482:9:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <elliptic_curve::dev::mock_curve::AffinePoint as group::CurveAffine>::to_curve
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/elliptic-curve-0.14.1/src/dev/mock_curve.rs:482:9
   4: repro_elliptic_curve_src_dev_mock_curve_rs_482_9_83ebcb::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
