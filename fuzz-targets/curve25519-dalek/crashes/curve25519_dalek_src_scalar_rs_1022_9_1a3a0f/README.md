# curve25519-dalek/src/scalar.rs:1022:9

- crate: https://github.com/dalek-cryptography/curve25519-dalek @ 97a020d185970839ed1d3f3a5c22aca816631786
- kind: panic
- location: `curve25519-dalek/src/scalar.rs:1022:9`
- last API call: `curve25519_dalek::RistrettoPoint::mul_base`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self[31] <= 127
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
curve25519-dalek = { git = "https://github.com/dalek-cryptography/curve25519-dalek", rev = "97a020d185970839ed1d3f3a5c22aca816631786", features = ["alloc", "digest", "ff", "group", "legacy_compatibility", "lizard", "precomputed-tables", "rand_core", "serde", "zeroize"] }
```

`src/main.rs`:

```rust
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;

fn main() {
    let _ = RistrettoPoint::mul_base(&<curve25519_dalek::Scalar as ::core::ops::Sub<curve25519_dalek::Scalar>>::sub(Scalar::from(0u32), Scalar::from_bits([0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8])));
}
```

Run with `cargo run` in this directory (package `repro_curve25519_dalek_src_scalar_rs_1022_9_1a3a0f`).

## Output

```
thread 'main' (1381325) panicked at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:1022:9:
assertion failed: self[31] <= 127
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <curve25519_dalek::scalar::Scalar>::as_radix_16
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:1022:9
   4: <curve25519_dalek::scalar::Scalar>::as_radix_2w
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:1100:25
   5: <curve25519_dalek::edwards::EdwardsBasepointTable as curve25519_dalek::traits::BasepointTable>::mul_base
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/edwards.rs:1232:32
   6: <&curve25519_dalek::edwards::EdwardsBasepointTable as core::ops::arith::Mul<&curve25519_dalek::scalar::Scalar>>::mul
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/edwards.rs:1258:22
   7: <&curve25519_dalek::scalar::Scalar as core::ops::arith::Mul<&curve25519_dalek::edwards::EdwardsBasepointTable>>::mul
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/edwards.rs:1268:17
   8: <&curve25519_dalek::scalar::Scalar as core::ops::arith::Mul<&curve25519_dalek::ristretto::RistrettoBasepointTable>>::mul
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/ristretto.rs:1096:24
   9: <curve25519_dalek::ristretto::RistrettoPoint>::mul_base
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/ristretto.rs:947:13
  10: repro_curve25519_dalek_src_scalar_rs_1022_9_1a3a0f::main
             at ./src/main.rs:5:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
