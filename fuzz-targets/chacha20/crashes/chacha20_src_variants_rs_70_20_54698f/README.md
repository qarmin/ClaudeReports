# chacha20/src/variants.rs:70:20

- crate: https://github.com/RustCrypto/stream-ciphers @ a2024dde67d449fe2bcb4feecda4bd3959a4704d
- kind: panic
- location: `chacha20/src/variants.rs:70:20`
- last API call: `chacha20::variants::Legacy::get_block_pos`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
chacha20 = { git = "https://github.com/RustCrypto/stream-ciphers", rev = "a2024dde67d449fe2bcb4feecda4bd3959a4704d", features = ["cipher", "legacy", "rng", "xchacha", "zeroize"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <chacha20::variants::Legacy as chacha20::variants::Variant>::get_block_pos(&[]);
}
```

Run with `cargo run` in this directory (package `repro_chacha20_src_variants_rs_70_20_54698f`).

## Output

```
thread 'main' (480335) panicked at /home/rafal/.cargo/git/checkouts/stream-ciphers-beca7f5cd8deeea7/a2024dd/chacha20/src/variants.rs:70:20:
index out of bounds: the len is 0 but the index is 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <chacha20::variants::Legacy as chacha20::variants::Variant>::get_block_pos
             at /home/rafal/.cargo/git/checkouts/stream-ciphers-beca7f5cd8deeea7/a2024dd/chacha20/src/variants.rs:70:20
   4: repro_chacha20_src_variants_rs_70_20_54698f::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
