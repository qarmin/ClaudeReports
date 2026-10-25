# out of memory in polling::Events::with_capacity

- crate: https://github.com/smol-rs/polling @ 35ba7a4fa8bf1e9f33711cb140a015d6bfe84c1b
- kind: out of memory
- last API call: `polling::Events::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling polling::Events::with_capacity
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
    let _ = Events::with_capacity(std::num::NonZero::new(1000000000usize).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_polling_events_with_capacity_fa98af`).

## Output

```
memory allocation of 12000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<rustix::event::epoll::Event>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<rustix::event::epoll::Event>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<rustix::event::epoll::Event>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <polling::epoll::Events>::with_capacity
             at /home/rafal/.cargo/git/checkouts/polling-740d07eed2780bfa/35ba7a4/src/epoll.rs:336:19
  10: <polling::Events>::with_capacity
             at /home/rafal/.cargo/git/checkouts/polling-740d07eed2780bfa/35ba7a4/src/lib.rs:874:21
  11: repro_out_of_memory_in_polling_events_with_capacity_fa98af::main
             at ./src/main.rs:4:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
