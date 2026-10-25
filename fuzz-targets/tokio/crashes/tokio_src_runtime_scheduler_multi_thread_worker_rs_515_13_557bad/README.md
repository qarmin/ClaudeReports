# tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13

- crate: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff
- kind: panic
- location: `tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13`
- last API call: `tokio::runtime::Builder::build`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
OS can't spawn worker thread: Resource temporarily unavailable (os error 11)
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
    let _ = builder.thread_stack_size(2147483648usize);
    let _ = builder.build();
}
```

Run with `cargo run` in this directory (package `repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_515_13_557bad`).

## Output

```
thread 'main' (1310620) panicked at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13:
OS can't spawn worker thread: Resource temporarily unavailable (os error 11)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <tokio::runtime::blocking::pool::Spawner>::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:386:17
   3: <tokio::runtime::handle::Handle>::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/handle.rs:239:39
   4: tokio::runtime::blocking::pool::spawn_blocking::<<tokio::runtime::scheduler::multi_thread::worker::Launch>::launch::{closure#0}, ()>
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/blocking/pool.rs:244:8
   5: <tokio::runtime::scheduler::multi_thread::worker::Launch>::launch
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/multi_thread/worker.rs:515:13
   6: <tokio::runtime::builder::Builder>::build_threaded_runtime
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:2236:20
   7: <tokio::runtime::builder::Builder>::build
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:1154:39
   8: repro_tokio_src_runtime_scheduler_multi_thread_worker_rs_515_13_557bad::main
             at ./src/main.rs:6:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
