# alloc-no-stdlib/src/stack_allocator.rs:52:13

- crate: https://github.com/dropbox/rust-brotli-decompressor @ 910099df8c358e58367593047f6b758920076ed6
- kind: panic
- location: `alloc-no-stdlib/src/stack_allocator.rs:52:13`
- source: https://docs.rs/crate/alloc-no-stdlib/3.0.0/source/src/stack_allocator.rs#52
- last API call: `brotli_decompressor::brotli_decode_prealloc`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
OOM
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli-decompressor = { git = "https://github.com/dropbox/rust-brotli-decompressor", rev = "910099df8c358e58367593047f6b758920076ed6" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = brotli_decompressor::brotli_decode_prealloc(&vec![], &mut vec![], &mut vec![], &mut vec![], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_alloc_no_stdlib_src_stack_allocator_rs_52_13_76ad39`).

## Output

```
thread 'main' (450544) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/alloc-no-stdlib-3.0.0/src/stack_allocator.rs:52:13:
OOM
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <alloc_no_stdlib::stack_allocator::StackAllocator<brotli_decompressor::huffman::HuffmanCode, brotli_decompressor::MemPool<brotli_decompressor::huffman::HuffmanCode>> as alloc_no_stdlib::stack_allocator::Allocator<brotli_decompressor::huffman::HuffmanCode>>::alloc_cell
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/alloc-no-stdlib-3.0.0/src/stack_allocator.rs:52:13
   4: <brotli_decompressor::state::BrotliState<alloc_no_stdlib::stack_allocator::StackAllocator<u8, brotli_decompressor::MemPool<u8>>, alloc_no_stdlib::stack_allocator::StackAllocator<u32, brotli_decompressor::MemPool<u32>>, alloc_no_stdlib::stack_allocator::StackAllocator<brotli_decompressor::huffman::HuffmanCode, brotli_decompressor::MemPool<brotli_decompressor::huffman::HuffmanCode>>>>::new
             at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/state.rs:525:52
   5: brotli_decompressor::brotli_decode_prealloc
             at /home/rafal/.cargo/git/checkouts/rust-brotli-decompressor-b7df0feb6e78a12a/910099d/src/lib.rs:395:5
   6: repro_alloc_no_stdlib_src_stack_allocator_rs_52_13_76ad39::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
