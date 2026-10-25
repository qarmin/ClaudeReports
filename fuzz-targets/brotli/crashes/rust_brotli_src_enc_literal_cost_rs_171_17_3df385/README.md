# rust-brotli/src/enc/literal_cost.rs:171:17

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/literal_cost.rs:171:17`
- last API call: `brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals;

fn main() {
    BrotliEstimateBitCostsForLiterals(0usize, 1usize, 0usize, &vec![35u8], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_literal_cost_rs_171_17_3df385`).

## Output

```
thread 'main' (1254157) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/literal_cost.rs:171:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::literal_cost::EstimateBitCostsForLiteralsUTF8
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/literal_cost.rs:171:17
   4: brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/literal_cost.rs:186:9
   5: repro_rust_brotli_src_enc_literal_cost_rs_171_17_3df385::main
             at ./src/main.rs:4:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
