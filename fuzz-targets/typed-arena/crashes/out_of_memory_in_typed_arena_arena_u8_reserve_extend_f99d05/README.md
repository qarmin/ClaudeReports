# out of memory in typed_arena::Arena<u8>::reserve_extend

- crate: https://github.com/SimonSapin/rust-typed-arena @ f1f26c3994dc881c35591cb629d8466df29e778a
- kind: out of memory
- last API call: `typed_arena::Arena<u8>::reserve_extend`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling typed_arena::Arena<u8>::reserve_extend
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
typed-arena = { git = "https://github.com/SimonSapin/rust-typed-arena", rev = "f1f26c3994dc881c35591cb629d8466df29e778a" }
```

`src/main.rs`:

```rust
fn main() {
    let arena = <typed_arena::Arena<u8>>::new();
    arena.reserve_extend(2147483648usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_typed_arena_arena_u8_reserve_extend_f99d05`).

## Output

```
memory allocation of 2147483648 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<u8>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <typed_arena::ChunkList<u8>>::reserve
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:560:53
  10: <typed_arena::Arena<u8>>::reserve_extend
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:387:20
  11: repro_out_of_memory_in_typed_arena_arena_u8_reserve_extend_f99d05::main
             at ./src/main.rs:3:11
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
