# fiat-rust/src/curve25519_32.rs:661:18

- crate: https://github.com/mit-plv/fiat-crypto @ 64e8f2791b77e080114512e8ff70f20660c611a6
- kind: panic
- location: `fiat-rust/src/curve25519_32.rs:661:18`
- last API call: `fiat_crypto::curve25519_32::fiat_25519_sub`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
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
    let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
    fiat_crypto::curve25519_32::fiat_25519_sub(&mut fiat_25519_loose_field_element, &fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 1u32, 0u32]), &fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));
}
```

Run with `cargo run` in this directory (package `repro_fiat_rust_src_curve25519_32_rs_661_18_6f8733`).

## Output

```
thread 'main' (1232328) panicked at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/curve25519_32.rs:661:18:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: fiat_crypto::curve25519_32::fiat_25519_sub
             at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/curve25519_32.rs:661:18
   4: repro_fiat_rust_src_curve25519_32_rs_661_18_6f8733::main
             at ./src/main.rs:3:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
