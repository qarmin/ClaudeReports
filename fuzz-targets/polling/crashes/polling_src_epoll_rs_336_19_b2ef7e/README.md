# polling/src/epoll.rs:336:19

- crate: https://github.com/smol-rs/polling @ 35ba7a4fa8bf1e9f33711cb140a015d6bfe84c1b
- kind: panic
- location: `polling/src/epoll.rs:336:19`
- last API call: `polling::Events::with_capacity`
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
polling = { git = "https://github.com/smol-rs/polling", rev = "35ba7a4fa8bf1e9f33711cb140a015d6bfe84c1b" }
```

`src/main.rs`:

```rust
use polling::Events;

fn main() {
    let _ = Events::with_capacity(std::num::NonZero::new(10000000000000000000usize).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_polling_src_epoll_rs_336_19_b2ef7e`).

## Output

```
thread 'main' (1394437) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   5: <alloc::raw_vec::RawVec<rustix::event::epoll::Event>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<rustix::event::epoll::Event>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<rustix::event::epoll::Event>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <polling::epoll::Events>::with_capacity
             at /home/rafal/.cargo/git/checkouts/polling-740d07eed2780bfa/35ba7a4/src/epoll.rs:336:19
   9: <polling::Events>::with_capacity
             at /home/rafal/.cargo/git/checkouts/polling-740d07eed2780bfa/35ba7a4/src/lib.rs:874:21
  10: repro_polling_src_epoll_rs_336_19_b2ef7e::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
