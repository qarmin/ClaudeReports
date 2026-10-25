# subtle/src/lib.rs:238:9

- crate: https://github.com/dalek-cryptography/subtle @ 369e7463e85921377a5f2df80aabcbbc6d57a930
- kind: panic
- location: `subtle/src/lib.rs:238:9`
- last API call: `subtle::Choice::from<u8>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: (input == 0u8) | (input == 1u8)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
subtle = { git = "https://github.com/dalek-cryptography/subtle", rev = "369e7463e85921377a5f2df80aabcbbc6d57a930" }
```

`src/main.rs`:

```rust
use subtle::Choice;

fn main() {
    let _ = Choice::from(100u8);
}
```

Run with `cargo run` in this directory (package `repro_subtle_src_lib_rs_238_9_9b5aec`).

## Output

```
thread 'main' (1402422) panicked at /home/rafal/.cargo/git/checkouts/subtle-4b70ec93f880be58/369e746/src/lib.rs:238:9:
assertion failed: (input == 0u8) | (input == 1u8)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <subtle::Choice as core::convert::From<u8>>::from
             at /home/rafal/.cargo/git/checkouts/subtle-4b70ec93f880be58/369e746/src/lib.rs:238:9
   4: repro_subtle_src_lib_rs_238_9_9b5aec::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
