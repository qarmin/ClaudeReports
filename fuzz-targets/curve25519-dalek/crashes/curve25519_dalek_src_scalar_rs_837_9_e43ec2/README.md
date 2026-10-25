# curve25519-dalek/src/scalar.rs:837:9

- crate: https://github.com/dalek-cryptography/curve25519-dalek @ 97a020d185970839ed1d3f3a5c22aca816631786
- kind: panic
- location: `curve25519-dalek/src/scalar.rs:837:9`
- last API call: `curve25519_dalek::Scalar::invert_batch_alloc`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: acc.pack() != Scalar::ZERO
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
curve25519-dalek = { git = "https://github.com/dalek-cryptography/curve25519-dalek", rev = "97a020d185970839ed1d3f3a5c22aca816631786" }
```

`src/main.rs`:

```rust
use curve25519_dalek::Scalar;

fn main() {
    let _ = Scalar::invert_batch_alloc(&mut vec![Scalar::from_bytes_mod_order([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8])]);
}
```

Run with `cargo run` in this directory (package `repro_curve25519_dalek_src_scalar_rs_837_9_e43ec2`).

## Output

```
thread 'main' (1380955) panicked at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:837:9:
assertion failed: acc.pack() != Scalar::ZERO
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <curve25519_dalek::scalar::Scalar>::invert_batch_internal
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:837:9
   4: <curve25519_dalek::scalar::Scalar>::invert_batch_alloc
             at /home/rafal/.cargo/git/checkouts/curve25519-dalek-47e4f7167d3fe2c0/97a020d/curve25519-dalek/src/scalar.rs:810:9
   5: repro_curve25519_dalek_src_scalar_rs_837_9_e43ec2::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
