# x11rb-protocol/src/protocol/xproto.rs:840:58

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xproto.rs:840:58`
- last API call: `x11rb_protocol::protocol::xproto::SetupFailed::serialize_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
`reason` has too many elements: TryFromIntError(PosOverflow)
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
use x11rb_protocol::protocol::xkb::SetKeyType;
use x11rb_protocol::protocol::xkb::VMod;
use x11rb_protocol::protocol::xproto::ModMask;
use x11rb_protocol::protocol::xproto::SetupFailed;

fn main() {
    let set_key_type = SetKeyType { mask: ModMask::M1, real_mods: ModMask::ANY, virtual_mods: VMod::M7, num_levels: 0u8, preserve: false, entries: vec![], preserve_entries: vec![] };
    let u8_v = set_key_type.n_map_entries();
    <x11rb_protocol::protocol::xproto::SetupFailed as x11rb_protocol::x11_utils::Serialize>::serialize_into(&SetupFailed { status: u8_v, protocol_major_version: 0u16, protocol_minor_version: 0u16, length: 0u16, reason: include_bytes!("data_0.bin").to_vec() }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xproto_rs_840_58_81b9bc`).

## Output

```
thread 'main' (321576) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xproto.rs:840:58:
`reason` has too many elements: TryFromIntError(PosOverflow)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<u8, core::num::error::TryFromIntError>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <x11rb_protocol::protocol::xproto::SetupFailed as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xproto.rs:840:58
   5: repro_x11rb_protocol_src_protocol_xproto_rs_840_58_81b9bc::main
             at ./src/main.rs:9:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
