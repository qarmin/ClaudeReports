# tokio/src/runtime/blocking/pool.rs:522:17

- crate: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff
- kind: panic
- location: `tokio/src/runtime/blocking/pool.rs:522:17`
- last API call: `tokio::runtime::Builder::build`
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
tokio = { git = "https://github.com/tokio-rs/tokio", rev = "cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff", features = ["bytes", "fs", "full", "io-std", "io-util", "libc", "macros", "mio", "net", "parking_lot", "process", "rt", "rt-multi-thread", "signal", "signal-hook-registry", "socket2", "sync", "test-util", "time", "tokio-macros", "tracing", "windows-sys"] }
```

`src/main.rs`:

```rust
use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.thread_name("\0".to_string());
    let _ = builder.build();
}
```

Run with `cargo run` in this directory (package `repro_tokio_src_runtime_blocking_pool_rs_522_17_62a0c6`).

## Output

```
thread 'main' (1314266) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/thread/thread.rs:24:40:
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
   8: std::thread::lifecycle::spawn_unchecked::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/lifecycle.rs:48:18
   9: <std::thread::builder::Builder>::spawn_unchecked::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/builder.rs:261:32
  10: <std::thread::builder::Builder>::spawn::<<tokio::runtime::blocking::pool::Spawner>::spawn_thread::{closure#0}, ()>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/builder.rs:191:23
  11: <tokio::runtime::blocking::pool::Spawner>::spawn_thread
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:522:17
  12: <tokio::runtime::blocking::pool::Spawner>::spawn_task::{closure#0}
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:482:32
  13: <tokio::runtime::blocking::pool::LockedImpl>::spawn_task::<<tokio::runtime::blocking::pool::Spawner>::spawn_task::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:628:13
  14: <tokio::runtime::blocking::pool::InnerImpl>::spawn_task::<<tokio::runtime::blocking::pool::Spawner>::spawn_task::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:559:39
  15: <tokio::runtime::blocking::pool::Spawner>::spawn_task
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:468:14
  16: <tokio::runtime::blocking::pool::Spawner>::spawn_blocking_inner::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:451:28
  17: <tokio::runtime::blocking::pool::Spawner>::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:373:18
  18: <tokio::runtime::handle::Handle>::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/handle.rs:239:39
  19: tokio::runtime::blocking::pool::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:244:8
  20: <tokio::runtime::scheduler::multi_thread::worker::Launch>::launch
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13
  21: <tokio::runtime::builder::Builder>::build_threaded_runtime
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:2236:20
  22: <tokio::runtime::builder::Builder>::build
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:1154:39
  23: repro_tokio_src_runtime_blocking_pool_rs_522_17_62a0c6::main
             at ./src/main.rs:6:21
  24: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
