# rust-threadpool/src/lib.rs:571:9

- crate: https://github.com/rust-threadpool/rust-threadpool @ a9dd15c278a8ab07d694efb169348138823bb89d
- kind: panic
- location: `rust-threadpool/src/lib.rs:571:9`
- last API call: `threadpool::ThreadPool::set_threads`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: num_threads >= 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
threadpool = { git = "https://github.com/rust-threadpool/rust-threadpool", rev = "a9dd15c278a8ab07d694efb169348138823bb89d" }
```

`src/main.rs`:

```rust
use threadpool::ThreadPool;

fn main() {
    let mut thread_pool = ThreadPool::default();
    thread_pool.set_threads(0usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_threadpool_src_lib_rs_571_9_69cf38`).

## Output

```
thread 'main' (1399810) panicked at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:571:9:
assertion failed: num_threads >= 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <threadpool::ThreadPool>::set_num_threads
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:571:9
   4: <threadpool::ThreadPool>::set_threads
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:527:14
   5: repro_rust_threadpool_src_lib_rs_571_9_69cf38::main
             at ./src/main.rs:5:17
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
