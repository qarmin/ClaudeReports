# rust-typed-arena/src/lib.rs:560:53

- crate: https://github.com/SimonSapin/rust-typed-arena @ f1f26c3994dc881c35591cb629d8466df29e778a
- kind: panic
- location: `rust-typed-arena/src/lib.rs:560:53`
- last API call: `typed_arena::Arena<u32>::reserve_extend`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
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
    let arena = <typed_arena::Arena<u32> as ::core::default::Default>::default();
    arena.reserve_extend(7252174369975988866usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_typed_arena_src_lib_rs_560_53_8a44ab`).

## Output

```
thread 'main' (1329946) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   5: <alloc::raw_vec::RawVec<u32>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<u32>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<u32>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <typed_arena::ChunkList<u32>>::reserve
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:560:53
   9: <typed_arena::Arena<u32>>::reserve_extend
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:387:20
  10: repro_rust_typed_arena_src_lib_rs_560_53_8a44ab::main
             at ./src/main.rs:3:11
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
