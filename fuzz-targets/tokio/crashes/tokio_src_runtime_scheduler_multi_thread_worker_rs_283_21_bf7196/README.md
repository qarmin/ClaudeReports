# tokio/src/runtime/scheduler/multi_thread/worker.rs:283:21

- crate: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff
- kind: panic
- location: `tokio/src/runtime/scheduler/multi_thread/worker.rs:283:21`
- last API call: `tokio::runtime::Builder::build`
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
tokio = { git = "https://github.com/tokio-rs/tokio", rev = "cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff", features = ["bytes", "fs", "full", "io-std", "io-util", "libc", "macros", "mio", "net", "parking_lot", "process", "rt", "rt-multi-thread", "signal", "signal-hook-registry", "socket2", "sync", "test-util", "time", "tokio-macros", "tracing", "windows-sys"] }
```

`src/main.rs`:

```rust
use tokio::runtime::Builder;

fn main() {
    let mut builder = Builder::new_multi_thread();
    let _ = builder.worker_threads(10000000000000000000usize);
    let _ = builder.build();
}
```

Run with `cargo run` in this directory (package `repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_283_21_bf7196`).

## Output

```
thread 'main' (436527) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   5: <alloc::raw_vec::RawVec<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<alloc::boxed::Box<tokio::runtime::scheduler::multi_thread::worker::Core>>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: tokio::runtime::scheduler::multi_thread::worker::create
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/multi_thread/worker.rs:283:21
   9: <tokio::runtime::scheduler::multi_thread::MultiThread>::new
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/multi_thread/mod.rs:67:32
  10: <tokio::runtime::builder::Builder>::build_threaded_runtime
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:2202:47
  11: <tokio::runtime::builder::Builder>::build
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:1154:39
  12: repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_283_21_bf7196::main
             at ./src/main.rs:6:21
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
