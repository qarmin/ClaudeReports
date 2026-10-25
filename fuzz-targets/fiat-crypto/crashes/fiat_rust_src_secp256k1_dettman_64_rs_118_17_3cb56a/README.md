# fiat-rust/src/secp256k1_dettman_64.rs:118:17

- crate: https://github.com/mit-plv/fiat-crypto @ 64e8f2791b77e080114512e8ff70f20660c611a6
- kind: panic
- location: `fiat-rust/src/secp256k1_dettman_64.rs:118:17`
- last API call: `fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fiat-crypto = { git = "https://github.com/mit-plv/fiat-crypto", rev = "64e8f2791b77e080114512e8ff70f20660c611a6" }
```

`src/main.rs`:

```rust
fn main() {
    fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square(&mut [0u64, 0u64, 0u64, 0u64, 0u64], &[0u64, 0u64, u64::MAX, 85u64, 5u64]);
}
```

Run with `cargo run` in this directory (package `repro_fiat_rust_src_secp256k1_dettman_64_rs_118_17_3cb56a`).

## Output

```
thread 'main' (1220440) panicked at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/secp256k1_dettman_64.rs:118:17:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_square
             at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/secp256k1_dettman_64.rs:118:17
   4: repro_fiat_rust_src_secp256k1_dettman_64_rs_118_17_3cb56a::main
             at ./src/main.rs:2:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
