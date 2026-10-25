# rust-typed-arena/src/lib.rs:558:14

- crate: https://github.com/SimonSapin/rust-typed-arena @ f1f26c3994dc881c35591cb629d8466df29e778a
- kind: panic
- location: `rust-typed-arena/src/lib.rs:558:14`
- last API call: `typed_arena::Arena<String>::reserve_extend`
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
    let arena = <typed_arena::Arena<String>>::new();
    let _ = <typed_arena::Arena<String> as ::core::default::Default>::default();
    arena.reserve_extend(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_typed_arena_src_lib_rs_558_14_a619c8`).

## Output

```
thread 'main' (1329893) panicked at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:558:14:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<usize>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <typed_arena::ChunkList<alloc::string::String>>::reserve
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:558:14
   6: <typed_arena::Arena<alloc::string::String>>::reserve_extend
             at /home/rafal/.cargo/git/checkouts/rust-typed-arena-05dd31959a0f9f57/f1f26c3/src/lib.rs:387:20
   7: repro_rust_typed_arena_src_lib_rs_558_14_a619c8::main
             at ./src/main.rs:4:11
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
