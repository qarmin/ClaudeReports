# x11rb-protocol/src/id_allocator.rs:54:23

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/id_allocator.rs:54:23`
- last API call: `x11rb_protocol::id_allocator::IdAllocator::update_xid_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
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
use x11rb_protocol::id_allocator::IdAllocator;
use x11rb_protocol::protocol::xc_misc::GetXIDRangeReply;
use x11rb_protocol::protocol::xproto::GetImageReply;

fn main() {
    let get_image_reply = GetImageReply::default();
    let u32_v = get_image_reply.length();
    let mut id_allocator = IdAllocator::new(u32_v, 1u32).unwrap();
    let _ = id_allocator.update_xid_range(&GetXIDRangeReply { sequence: 0u16, length: u32_v, start_id: 10u32, count: u32::MAX });
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_id_allocator_rs_54_23_d9f484`).

## Output

```
thread 'main' (321909) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/id_allocator.rs:54:23:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <x11rb_protocol::id_allocator::IdAllocator>::update_xid_range
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/id_allocator.rs:54:23
   4: repro_x11rb_protocol_src_id_allocator_rs_54_23_d9f484::main
             at ./src/main.rs:9:26
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
