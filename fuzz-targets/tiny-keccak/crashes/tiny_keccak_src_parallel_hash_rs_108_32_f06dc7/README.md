# tiny-keccak/src/parallel_hash.rs:108:32

- crate: tiny-keccak 2.0.2
- kind: panic
- location: `tiny-keccak/src/parallel_hash.rs:108:32`
- source: https://docs.rs/crate/tiny-keccak/2.0.2/source/src/parallel_hash.rs#108
- last API call: `tiny_keccak::ParallelHash::update`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to divide by zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-keccak = { version = "=2.0.2", features = ["cshake", "fips202", "k12", "keccak", "kmac", "parallel_hash", "sha3", "shake", "sp800", "tuple_hash"] }
```

`src/main.rs`:

```rust
use tiny_keccak::ParallelHash;

fn main() {
    let mut parallel_hash = ParallelHash::v256(&vec![], 0usize);
    <tiny_keccak::ParallelHash as tiny_keccak::Hasher>::update(&mut parallel_hash, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_tiny_keccak_src_parallel_hash_rs_108_32_f06dc7`).

## Output

```
thread 'main' (569259) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-keccak-2.0.2/src/parallel_hash.rs:108:32:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <tiny_keccak::parallel_hash::ParallelHash as tiny_keccak::Hasher>::update
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tiny-keccak-2.0.2/src/parallel_hash.rs:108:32
   4: repro_tiny_keccak_src_parallel_hash_rs_108_32_f06dc7::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
