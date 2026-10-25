# fiat-rust/src/poly1305_64.rs:275:61

- crate: https://github.com/mit-plv/fiat-crypto @ 64e8f2791b77e080114512e8ff70f20660c611a6
- kind: panic
- location: `fiat-rust/src/poly1305_64.rs:275:61`
- last API call: `fiat_crypto::poly1305_64::fiat_poly1305_carry_mul`
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
    let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64, 0u64, 0u64]);
    fiat_crypto::poly1305_64::fiat_poly1305_carry_mul(&mut fiat_poly1305_tight_field_element, &fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 0u64, 0u64]), &fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([1u64, 10000000000000000000u64, 90u64]));
}
```

Run with `cargo run` in this directory (package `repro_fiat_rust_src_poly1305_64_rs_275_61_5c267a`).

## Output

```
thread 'main' (1231832) panicked at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/poly1305_64.rs:275:61:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: fiat_crypto::poly1305_64::fiat_poly1305_carry_mul
             at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/poly1305_64.rs:275:61
   4: repro_fiat_rust_src_poly1305_64_rs_275_61_5c267a::main
             at ./src/main.rs:3:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
