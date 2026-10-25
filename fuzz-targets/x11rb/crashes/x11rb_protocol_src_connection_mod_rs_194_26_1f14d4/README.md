# x11rb-protocol/src/connection/mod.rs:194:26

- crate: x11rb 0.14.0
- kind: panic
- location: `x11rb-protocol/src/connection/mod.rs:194:26`
- source: https://docs.rs/crate/x11rb-protocol/0.14.0/source/src/connection/mod.rs#194
- last API call: `x11rb_protocol::connection::Connection::enqueue_packet`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
x11rb = { version = "=0.14.0" }
x11rb-protocol = { version = "=0.14.0" }
```

`src/main.rs`:

```rust
use x11rb_protocol::connection::Connection;

fn main() {
    let mut connection = Connection::default();
    connection.enqueue_packet(vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_connection_mod_rs_194_26_1f14d4`).

## Output

```
thread 'main' (3280673) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x11rb-protocol-0.14.0/src/connection/mod.rs:194:26:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[u8] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<u8> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <x11rb_protocol::connection::Connection>::enqueue_packet
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x11rb-protocol-0.14.0/src/connection/mod.rs:194:26
   7: repro_x11rb_protocol_src_connection_mod_rs_194_26_1f14d4::main
             at ./src/main.rs:5:16
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
