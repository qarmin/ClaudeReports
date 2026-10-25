# rust-threadpool/src/lib.rs:739:10

- crate: https://github.com/rust-threadpool/rust-threadpool @ a9dd15c278a8ab07d694efb169348138823bb89d
- kind: panic
- location: `rust-threadpool/src/lib.rs:739:10`
- last API call: `threadpool::ThreadPool::new_with_name`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
thread name may not contain interior null bytes: NulError(0, [0])
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
    let _ = ThreadPool::new_with_name("\0".to_string(), 1usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_threadpool_src_lib_rs_739_10_6040ca`).

## Output

```
thread 'main' (1399937) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/thread/thread.rs:24:40:
thread name may not contain interior null bytes: NulError(0, [0])
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<alloc::ffi::c_str::CString, alloc::ffi::c_str::NulError>>::expect
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1184:23
   4: <std::thread::thread::thread_name_string::ThreadNameString as core::convert::From<alloc::string::String>>::from
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/thread/thread.rs:24:40
   5: <<std::thread::thread::thread_name_string::ThreadNameString as core::convert::From<alloc::string::String>>::from as core::ops::function::FnOnce<(alloc::string::String,)>>::call_once
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/function.rs:250:5
   6: <core::option::Option<alloc::string::String>>::map::<std::thread::thread::thread_name_string::ThreadNameString, <std::thread::thread::thread_name_string::ThreadNameString as core::convert::From<alloc::string::String>>::from>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:1163:29
   7: <std::thread::thread::Thread>::new
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/thread/thread.rs:94:25
   8: std::thread::lifecycle::spawn_unchecked::<threadpool::spawn_in_pool::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/lifecycle.rs:48:18
   9: <std::thread::builder::Builder>::spawn_unchecked::<threadpool::spawn_in_pool::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/builder.rs:261:32
  10: <std::thread::builder::Builder>::spawn::<threadpool::spawn_in_pool::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/builder.rs:191:23
  11: threadpool::spawn_in_pool
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:739:10
  12: <threadpool::Builder>::build
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:301:13
  13: <threadpool::ThreadPool>::with_name
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:401:14
  14: <threadpool::ThreadPool>::new_with_name
             at /home/rafal/.cargo/git/checkouts/rust-threadpool-77c20427aa583cae/a9dd15c/src/lib.rs:408:9
  15: repro_rust_threadpool_src_lib_rs_739_10_6040ca::main
             at ./src/main.rs:4:13
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
