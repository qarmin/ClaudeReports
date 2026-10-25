# elliptic-curve/src/dev/mock_curve.rs:453:9

- crate: elliptic-curve 0.14.1
- kind: panic
- location: `elliptic-curve/src/dev/mock_curve.rs:453:9`
- source: https://docs.rs/crate/elliptic-curve/0.14.1/source/src/dev/mock_curve.rs#453
- last API call: `elliptic_curve::dev::mock_curve::AffinePoint::y`
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
use elliptic_curve::dev::mock_curve::AffinePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::y(&<elliptic_curve::point::NonIdentity<elliptic_curve::dev::mock_curve::AffinePoint> as ::core::convert::TryFrom<elliptic_curve::dev::mock_curve::AffinePoint>>::try_from(AffinePoint::Generator).unwrap().to_point());
}
```

Run with `cargo run` in this directory (package `repro_elliptic_curve_src_dev_mock_curve_rs_453_9_aca93e`).

## Output

```
thread 'main' (2581785) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/elliptic-curve-0.14.1/src/dev/mock_curve.rs:453:9:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::y
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/elliptic-curve-0.14.1/src/dev/mock_curve.rs:453:9
   4: repro_elliptic_curve_src_dev_mock_curve_rs_453_9_aca93e::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
