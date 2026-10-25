# thread_local-rs/src/lib.rs:185:35

- crate: https://github.com/Amanieu/thread_local-rs @ 69fb0508498fc4484cf5696ed16f2c9d6c50f9d4
- kind: panic
- location: `thread_local-rs/src/lib.rs:185:35`
- last API call: `thread_local::ThreadLocal<String>::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 64 out of range for slice of length 63
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
thread_local = { git = "https://github.com/Amanieu/thread_local-rs", rev = "69fb0508498fc4484cf5696ed16f2c9d6c50f9d4" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <thread_local::ThreadLocal<String>>::with_capacity(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_thread_local_rs_src_lib_rs_185_35_d517c9`).

## Output

```
thread 'main' (1422444) panicked at /home/rafal/.cargo/git/checkouts/thread_local-rs-dade8f86ab3e503e/69fb050/src/lib.rs:185:35:
range end index 64 out of range for slice of length 63
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[core::sync::atomic::Atomic<*mut thread_local::Entry<alloc::string::String>>]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:427:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[core::sync::atomic::Atomic<*mut thread_local::Entry<alloc::string::String>>]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:509:23
   5: <[core::sync::atomic::Atomic<*mut thread_local::Entry<alloc::string::String>>] as core::ops::index::IndexMut<core::ops::range::RangeTo<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   6: <[core::sync::atomic::Atomic<*mut thread_local::Entry<alloc::string::String>>; 63] as core::ops::index::IndexMut<core::ops::range::RangeTo<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:403:9
   7: <thread_local::ThreadLocal<alloc::string::String>>::with_capacity
             at /home/rafal/.cargo/git/checkouts/thread_local-rs-dade8f86ab3e503e/69fb050/src/lib.rs:185:35
   8: repro_thread_local_rs_src_lib_rs_185_35_d517c9::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
