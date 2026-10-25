# x11rb-protocol/src/protocol/xinput.rs:7793:9

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xinput.rs:7793:9`
- last API call: `x11rb_protocol::protocol::xinput::DeviceStateDataResolution::serialize_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: `resolution_max` has an incorrect length
  left: 0
 right: 5
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
use x11rb_protocol::protocol::xinput::DeviceStateDataResolution;
use x11rb_protocol::protocol::xselinux::ListSelectionsReply;

fn main() {
    let list_selections_reply = ListSelectionsReply::default();
    let u32_v = list_selections_reply.selections_len();
    <x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceStateDataResolution { resolution_values: vec![0u32, 0u32, 0u32, 0u32, 0u32], resolution_min: vec![0u32, 0u32, 0u32, u32_v, 1u32], resolution_max: vec![] }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xinput_rs_7793_9_b9bb5a`).

## Output

```
thread 'main' (325966) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:7793:9:
assertion `left == right` failed: `resolution_max` has an incorrect length
  left: 0
 right: 5
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <x11rb_protocol::protocol::xinput::DeviceStateDataResolution as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xinput.rs:7793:9
   5: repro_x11rb_protocol_src_protocol_xinput_rs_7793_9_b9bb5a::main
             at ./src/main.rs:7:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
