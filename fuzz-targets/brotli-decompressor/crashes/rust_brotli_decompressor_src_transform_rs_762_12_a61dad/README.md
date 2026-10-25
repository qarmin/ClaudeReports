# rust-brotli-decompressor/src/transform.rs:762:12

- crate: https://github.com/dropbox/rust-brotli-decompressor @ 910099df8c358e58367593047f6b758920076ed6
- kind: panic
- location: `rust-brotli-decompressor/src/transform.rs:762:12`
- last API call: `brotli_decompressor::transform::TransformDictionaryWord`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsafe precondition(s) violated: ptr::offset requires the address calculation to not overflow

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
    let _ = TransformDictionaryWord(&mut vec![], &vec![], -100000000i32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_decompressor_src_transform_rs_762_12_a61dad`).

## Output

```
thread 'main' (1336430) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/transform.rs:762:12:
range start index 18446744073609551616 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[u8] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: brotli_decompressor::transform::TransformDictionaryWord
             at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/memory.rs:34:17
   6: repro_rust_brotli_decompressor_src_transform_rs_762_12_a61dad::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
