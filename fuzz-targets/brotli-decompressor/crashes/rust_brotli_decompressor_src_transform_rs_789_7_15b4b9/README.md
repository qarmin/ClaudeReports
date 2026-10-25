# rust-brotli-decompressor/src/transform.rs:789:7

- crate: https://github.com/dropbox/rust-brotli-decompressor @ 910099df8c358e58367593047f6b758920076ed6
- kind: panic
- location: `rust-brotli-decompressor/src/transform.rs:789:7`
- last API call: `brotli_decompressor::transform::TransformDictionaryWord`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsafe precondition(s) violated: slice::get_unchecked_mut requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli-decompressor = { git = "https://github.com/dropbox/rust-brotli-decompressor", rev = "910099df8c358e58367593047f6b758920076ed6" }
```

`src/main.rs`:

```rust
use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let _ = TransformDictionaryWord(&mut vec![], &vec![], 0i32, 1i32);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_decompressor_src_transform_rs_789_7_15b4b9`).

## Output

```
thread 'main' (1336563) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/transform.rs:789:7:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli_decompressor::transform::TransformDictionaryWord
             at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/memory.rs:49:8
   4: repro_rust_brotli_decompressor_src_transform_rs_789_7_15b4b9::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
