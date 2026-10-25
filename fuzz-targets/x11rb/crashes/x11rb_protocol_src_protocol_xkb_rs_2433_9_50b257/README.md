# x11rb-protocol/src/protocol/xkb.rs:2433:9

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xkb.rs:2433:9`
- last API call: `x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry::serialize_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: `alignment_pad` has an incorrect length
  left: 0
 right: 2
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
use x11rb_protocol::protocol::xkb::CountedString16;
use x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry;

fn main() {
    <x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetKbdByNameRepliesGeometry { geometry_type: 0u8, geometry_device_id: 0u8, geometry_sequence: 0u16, geometry_length: 0u32, name: 0u32, geometry_found: false, width_mm: 0u16, height_mm: 0u16, n_properties: 0u16, n_colors: 0u16, n_shapes: 0u16, n_sections: 0u16, n_doodads: 0u16, n_key_aliases: 0u16, base_color_ndx: 0u8, label_color_ndx: 0u8, label_font: CountedString16::default() }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xkb_rs_2433_9_50b257`).

## Output

```
thread 'main' (322676) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:2433:9:
assertion `left == right` failed: `alignment_pad` has an incorrect length
  left: 0
 right: 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <x11rb_protocol::protocol::xkb::CountedString16 as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:2433:9
   5: <x11rb_protocol::protocol::xkb::GetKbdByNameRepliesGeometry as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:12188:25
   6: repro_x11rb_protocol_src_protocol_xkb_rs_2433_9_50b257::main
             at ./src/main.rs:5:5
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
