# x11rb-protocol/src/packet_reader.rs:83:9

- crate: x11rb 0.14.0
- kind: panic
- location: `x11rb-protocol/src/packet_reader.rs:83:9`
- source: https://docs.rs/crate/x11rb-protocol/0.14.0/source/src/packet_reader.rs#83
- last API call: `x11rb_protocol::packet_reader::PacketReader::advance`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.already_read <= self.pending_packet.len()
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
use x11rb_protocol::packet_reader::PacketReader;

fn main() {
    let mut packet_reader = PacketReader::default();
    let _ = packet_reader.advance(10000usize);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_packet_reader_rs_83_9_87eda5`).

## Output

```
thread 'main' (3283182) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x11rb-protocol-0.14.0/src/packet_reader.rs:83:9:
assertion failed: self.already_read <= self.pending_packet.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <x11rb_protocol::packet_reader::PacketReader>::advance
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x11rb-protocol-0.14.0/src/packet_reader.rs:83:9
   4: repro_x11rb_protocol_src_packet_reader_rs_83_9_87eda5::main
             at ./src/main.rs:5:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
