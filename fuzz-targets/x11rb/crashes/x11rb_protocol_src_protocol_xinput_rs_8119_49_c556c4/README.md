# x11rb-protocol/src/protocol/xinput.rs:8119:49

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xinput.rs:8119:49`
- last API call: `x11rb_protocol::protocol::xinput::DeviceState::serialize_into`
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
use x11rb_protocol::protocol::xinput::DeviceState;
use x11rb_protocol::protocol::xinput::DeviceStateData;
use x11rb_protocol::protocol::xinput::FeedbackCtlDataString;

fn main() {
    let feedback_ctl_data_string = FeedbackCtlDataString { keysyms: vec![] };
    let u16_v = feedback_ctl_data_string.num_keysyms();
    <x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceState { len: u16_v, data: DeviceStateData::InvalidValue(1u16) }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xinput_rs_8119_49_c556c4`).

## Output

```
thread 'main' (324401) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:8119:49:
attempted to serialize invalid switch case
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <x11rb_protocol::protocol::xinput::DeviceStateData>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:8119:49
   3: <x11rb_protocol::protocol::xinput::DeviceState as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:8165:19
   4: repro_x11rb_protocol_src_protocol_xinput_rs_8119_49_c556c4::main
             at ./src/main.rs:8:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
