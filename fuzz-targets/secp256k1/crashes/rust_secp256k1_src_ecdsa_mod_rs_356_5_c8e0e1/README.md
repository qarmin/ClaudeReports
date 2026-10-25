# rust-secp256k1/src/ecdsa/mod.rs:356:5

- crate: https://git.rust-bitcoin.org/rust-bitcoin/rust-secp256k1 @ ef04e0bd5928ff3fcb1834ed56e465882cdda385
- kind: panic
- location: `rust-secp256k1/src/ecdsa/mod.rs:356:5`
- last API call: `secp256k1::ecdsa::Signature::serialize_der`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: bytes_to_grind <= 71
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
secp256k1 = { git = "https://git.rust-bitcoin.org/rust-bitcoin/rust-secp256k1", rev = "ef04e0bd5928ff3fcb1834ed56e465882cdda385" }
```

`src/main.rs`:

```rust
use secp256k1::Message;
use secp256k1::SecretKey;

fn main() {
    let _ = secp256k1::ecdsa::sign_grind_r(Message::from_digest([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]), &SecretKey::from_byte_array([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]).unwrap(), usize::MAX).serialize_der();
}
```

Run with `cargo run` in this directory (package `repro_rust_secp256k1_src_ecdsa_mod_rs_356_5_c8e0e1`).

## Output

```
thread 'main' (1990927) panicked at /home/rafal/.cargo/git/checkouts/rust-secp256k1-73186b7fc25f0c67/ef04e0b/src/ecdsa/mod.rs:356:5:
assertion failed: bytes_to_grind <= 71
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: secp256k1::ecdsa::sign_grind_r::<secp256k1::Message>
             at /home/rafal/.cargo/git/checkouts/rust-secp256k1-73186b7fc25f0c67/ef04e0b/src/ecdsa/mod.rs:356:5
   4: repro_rust_secp256k1_src_ecdsa_mod_rs_356_5_c8e0e1::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
