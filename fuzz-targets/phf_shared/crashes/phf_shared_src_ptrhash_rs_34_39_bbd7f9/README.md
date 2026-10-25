# phf_shared/src/ptrhash.rs:34:39

- crate: phf_shared 0.14.0
- kind: panic
- location: `phf_shared/src/ptrhash.rs:34:39`
- source: https://docs.rs/crate/phf_shared/0.14.0/source/src/ptrhash.rs#34
- last API call: `phf_shared::ptrhash::get_index`
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
phf_shared = { version = "=0.14.0", features = ["ptrhash", "std", "uncased", "unicase"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = phf_shared::ptrhash::get_index(0u64, 0u64, &vec![], &[], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_phf_shared_src_ptrhash_rs_34_39_bbd7f9`).

## Output

```
thread 'main' (3384145) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/phf_shared-0.14.0/src/ptrhash.rs:34:39:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: phf_shared::ptrhash::get_index
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/phf_shared-0.14.0/src/ptrhash.rs:34:39
   4: repro_phf_shared_src_ptrhash_rs_34_39_bbd7f9::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
