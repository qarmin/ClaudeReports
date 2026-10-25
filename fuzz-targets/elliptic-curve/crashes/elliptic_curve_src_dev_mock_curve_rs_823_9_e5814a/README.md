# elliptic-curve/src/dev/mock_curve.rs:823:9

- crate: https://github.com/RustCrypto/traits @ 08780da1ae8ddad136b29fe77a93731d40c7ef38
- kind: panic
- location: `elliptic-curve/src/dev/mock_curve.rs:823:9`
- last API call: `elliptic_curve::dev::mock_curve::ProjectivePoint::add<elliptic_curve::dev::mock_curve::ProjectivePoint>`
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
elliptic-curve = { git = "https://github.com/RustCrypto/traits", rev = "08780da1ae8ddad136b29fe77a93731d40c7ef38", features = ["alloc", "arithmetic", "dev", "digest", "ecdh", "ff", "getrandom", "group", "pem", "pkcs8", "sec1", "serde", "std"] }
```

`src/main.rs`:

```rust
use elliptic_curve::dev::mock_curve::ProjectivePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Add<elliptic_curve::dev::mock_curve::ProjectivePoint>>::add(ProjectivePoint::from(<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::generator()), ProjectivePoint::from(<elliptic_curve::point::NonIdentity<elliptic_curve::dev::mock_curve::ProjectivePoint> as ::core::convert::TryFrom<elliptic_curve::dev::mock_curve::ProjectivePoint>>::try_from(<elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::generator()).unwrap()));
}
```

Run with `cargo run` in this directory (package `repro_elliptic_curve_src_dev_mock_curve_rs_823_9_e5814a`).

## Output

```
thread 'main' (371099) panicked at /home/rafal/.cargo/git/checkouts/traits-897a5f44fa539ef9/08780da/elliptic-curve/src/dev/mock_curve.rs:823:9:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Add>::add
             at /home/rafal/.cargo/git/checkouts/traits-897a5f44fa539ef9/08780da/elliptic-curve/src/dev/mock_curve.rs:823:9
   4: repro_elliptic_curve_src_dev_mock_curve_rs_823_9_e5814a::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
