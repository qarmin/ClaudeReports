# bs58-rs/src/decode.rs:657:17

- crate: https://github.com/Nullus157/bs58-rs @ e4a65a9ec641f26b61fb20518b51e83efd6352a0
- kind: panic
- location: `bs58-rs/src/decode.rs:657:17`
- last API call: `bs58::decode::Error::unwrap_const`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
provided string contained non-ascii character
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bs58 = { git = "https://github.com/Nullus157/bs58-rs", rev = "e4a65a9ec641f26b61fb20518b51e83efd6352a0" }
```

`src/main.rs`:

```rust
use bs58::decode::Error;

fn main() {
    let error = Error::NonAsciiCharacter { index: 0usize };
    let _ = error.unwrap_const();
}
```

Run with `cargo run` in this directory (package `repro_bs58_rs_src_decode_rs_657_17_235d57`).

## Output

```
thread 'main' (1329135) panicked at /home/rafal/.cargo/git/checkouts/bs58-rs-0a14f6d69120019d/e4a65a9/src/decode.rs:657:17:
provided string contained non-ascii character
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <bs58::decode::Error>::unwrap_const
             at /home/rafal/.cargo/git/checkouts/bs58-rs-0a14f6d69120019d/e4a65a9/src/decode.rs:657:17
   3: repro_bs58_rs_src_decode_rs_657_17_235d57::main
             at ./src/main.rs:5:19
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
