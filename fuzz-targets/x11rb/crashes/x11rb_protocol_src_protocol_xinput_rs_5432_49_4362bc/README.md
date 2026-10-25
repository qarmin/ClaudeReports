# x11rb-protocol/src/protocol/xinput.rs:5432:49

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xinput.rs:5432:49`
- last API call: `x11rb_protocol::protocol::xinput::ChangeFeedbackControlRequest::serialize`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempted to serialize invalid switch case
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
x11rb = { git = "https://github.com/psychon/x11rb", rev = "86c4252fe023fddd523f0ac3c7c07c40ba266ce0", features = ["all-extensions", "allow-unsafe-code", "as-raw-xcb-connection", "composite", "cursor", "damage", "dbe", "dl-libxcb", "dpms", "dri2", "dri3", "extra-traits", "glx", "image", "libc", "libloading", "once_cell", "present", "randr", "raw-window-handle", "record", "render", "request-parsing", "res", "resource_manager", "screensaver", "shape", "shm", "sync", "tracing", "xcursor", "xevie", "xf86dri", "xf86vidmode", "xfixes", "xinerama", "xinput", "xkb", "xprint", "xselinux", "xtest", "xv", "xvmc"] }
x11rb-protocol = { git = "https://github.com/psychon/x11rb", rev = "86c4252fe023fddd523f0ac3c7c07c40ba266ce0" }
```

`src/main.rs`:

```rust
use x11rb_protocol::protocol::xinput::ChangeFeedbackControlMask;
use x11rb_protocol::protocol::xinput::ChangeFeedbackControlRequest;
use x11rb_protocol::protocol::xinput::FeedbackCtl;
use x11rb_protocol::protocol::xinput::FeedbackCtlData;

fn main() {
    let change_feedback_control_request = ChangeFeedbackControlRequest { mask: ChangeFeedbackControlMask::PERCENT, device_id: 0u8, feedback_id: 0u8, feedback: FeedbackCtl { feedback_id: 0u8, len: 0u16, data: FeedbackCtlData::InvalidValue(0u8) } };
    let _ = change_feedback_control_request.clone().serialize(0u8);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xinput_rs_5432_49_4362bc`).

## Output

```
thread 'main' (321836) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:5432:49:
attempted to serialize invalid switch case
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <x11rb_protocol::protocol::xinput::FeedbackCtlData>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:5432:49
   3: <x11rb_protocol::protocol::xinput::FeedbackCtl as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:5482:19
   4: <x11rb_protocol::protocol::xinput::FeedbackCtl as x11rb_protocol::x11_utils::Serialize>::serialize
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:5473:14
   5: <x11rb_protocol::protocol::xinput::ChangeFeedbackControlRequest>::serialize
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:5590:44
   6: repro_x11rb_protocol_src_protocol_xinput_rs_5432_49_4362bc::main
             at ./src/main.rs:8:53
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
