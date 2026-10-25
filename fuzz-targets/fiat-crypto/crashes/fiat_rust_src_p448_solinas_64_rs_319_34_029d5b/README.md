# fiat-rust/src/p448_solinas_64.rs:319:34

- crate: https://github.com/mit-plv/fiat-crypto @ 64e8f2791b77e080114512e8ff70f20660c611a6
- kind: panic
- location: `fiat-rust/src/p448_solinas_64.rs:319:34`
- last API call: `fiat_crypto::p448_solinas_64::fiat_p448_carry_mul`
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
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 1u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, u64::MAX, 68u64, 14988368381062527468u64, 3383299671908264572u64, 0u64, 16u64]), &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([2u64, 1u64, 18446744073709551614u64, 15035455111019105998u64, u64::MAX, u64::MAX, 13170709496967601630u64, 1u64]));
}
```

Run with `cargo run` in this directory (package `repro_fiat_rust_src_p448_solinas_64_rs_319_34_029d5b`).

## Output

```
thread 'main' (1218749) panicked at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/p448_solinas_64.rs:319:34:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: fiat_crypto::p448_solinas_64::fiat_p448_carry_mul
             at /home/rafal/.cargo/git/checkouts/fiat-crypto-0bcc970f02d2898e/64e8f27/fiat-rust/src/p448_solinas_64.rs:319:34
   4: repro_fiat_rust_src_p448_solinas_64_rs_319_34_029d5b::main
             at ./src/main.rs:3:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
