# rust-brotli/src/enc/prior_eval.rs:253:37

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/prior_eval.rs:253:37`
- last API call: `brotli::enc::prior_eval::CMPrior::lookup_lin`
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
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior>::lookup_lin(0u8, 0u8, usize::MAX, Some(15u8));
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_prior_eval_rs_253_37_b2b4b1`).

## Output

```
thread 'main' (1253838) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/prior_eval.rs:253:37:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior>::lookup_lin
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/prior_eval.rs:253:37
   4: repro_rust_brotli_src_enc_prior_eval_rs_253_37_b2b4b1::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
