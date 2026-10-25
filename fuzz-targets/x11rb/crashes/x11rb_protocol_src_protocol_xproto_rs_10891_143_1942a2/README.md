# x11rb-protocol/src/protocol/xproto.rs:10891:143

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xproto.rs:10891:143`
- last API call: `x11rb_protocol::protocol::xproto::GetPropertyReply::serialize_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
x11rb = { git = "https://github.com/psychon/x11rb", rev = "86c4252fe023fddd523f0ac3c7c07c40ba266ce0" }
x11rb-protocol = { git = "https://github.com/psychon/x11rb", rev = "86c4252fe023fddd523f0ac3c7c07c40ba266ce0" }
```

`src/main.rs`:

```rust
use x11rb_protocol::protocol::xproto::GetPropertyReply;

fn main() {
    <x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetPropertyReply { format: 100u8, sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, value_len: 1000000000u32, value: vec![] }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xproto_rs_10891_143_1942a2`).

## Output

```
thread 'main' (326199) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xproto.rs:10891:143:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<u32>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xproto.rs:10891:143
   6: repro_x11rb_protocol_src_protocol_xproto_rs_10891_143_1942a2::main
             at ./src/main.rs:4:5
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
