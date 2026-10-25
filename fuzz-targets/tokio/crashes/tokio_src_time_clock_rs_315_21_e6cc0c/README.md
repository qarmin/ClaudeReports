# tokio/src/time/clock.rs:315:21

- crate: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff
- kind: panic
- location: `tokio/src/time/clock.rs:315:21`
- last API call: `tokio::runtime::Builder::build`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
`time::pause()` requires the `current_thread` Tokio runtime. This is the default Runtime used by `#[tokio::test].
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
    let _ = builder.start_paused(true);
    let _ = builder.build();
}
```

Run with `cargo run` in this directory (package `repro_tokio_src_time_clock_rs_315_21_e6cc0c`).

## Output

```
thread 'main' (436467) panicked at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/time/clock.rs:315:21:
`time::pause()` requires the `current_thread` Tokio runtime. This is the default Runtime used by `#[tokio::test].
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:259:5
   3: <tokio::time::clock::Clock>::new
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/time/clock.rs:315:21
   4: tokio::runtime::driver::create_clock
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/driver.rs:316:9
   5: <tokio::runtime::driver::Driver>::new
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/driver.rs:52:21
   6: <tokio::runtime::builder::Builder>::build_threaded_runtime
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:2191:43
   7: <tokio::runtime::builder::Builder>::build
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/builder.rs:1154:39
   8: repro_tokio_src_time_clock_rs_315_21_e6cc0c::main
             at ./src/main.rs:6:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
