# tokio/src/net/unix/stream.rs:936:18

- crate: https://github.com/tokio-rs/tokio @ cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff
- kind: panic
- location: `tokio/src/net/unix/stream.rs:936:18`
- last API call: `tokio::net::UnixStream::pair`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
there is no reactor running, must be called from the context of a Tokio 1.x runtime
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tokio = { git = "https://github.com/tokio-rs/tokio", rev = "cdd9dae3aa5056e23dc4d5bc4f852ab8fbf27aff", features = ["bytes", "fs", "full", "io-std", "io-util", "libc", "macros", "mio", "net", "parking_lot", "process", "rt", "rt-multi-thread", "signal", "signal-hook-registry", "socket2", "sync", "test-util", "time", "tokio-macros", "tracing", "windows-sys"] }
```

`src/main.rs`:

```rust
use tokio::net::UnixStream;

fn main() {
    let _ = UnixStream::pair();
}
```

Run with `cargo run` in this directory (package `repro_tokio_src_net_unix_stream_rs_936_18_33474f`).

## Output

```
thread 'main' (1310404) panicked at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/net/unix/stream.rs:936:18:
there is no reactor running, must be called from the context of a Tokio 1.x runtime
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<tokio::runtime::handle::TryCurrentError>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:259:5
   3: <tokio::runtime::scheduler::Handle>::current
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/runtime/scheduler/mod.rs:92:27
   4: <tokio::io::poll_evented::PollEvented<mio::net::uds::stream::UnixStream>>::new_with_interest
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/io/poll_evented.rs:111:58
   5: <tokio::io::poll_evented::PollEvented<mio::net::uds::stream::UnixStream>>::new
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/io/poll_evented.rs:90:9
   6: <tokio::net::unix::stream::UnixStream>::new
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/net/unix/stream.rs:936:18
   7: <tokio::net::unix::stream::UnixStream>::pair
             at /home/rafal/.cargo/git/checkouts/tokio-055782600e5cc385/cdd9dae/tokio/src/net/unix/stream.rs:915:17
   8: repro_tokio_src_net_unix_stream_rs_936_18_33474f::main
             at ./src/main.rs:4:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
