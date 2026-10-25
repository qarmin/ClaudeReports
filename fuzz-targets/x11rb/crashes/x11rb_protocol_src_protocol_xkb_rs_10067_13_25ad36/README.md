# x11rb-protocol/src/protocol/xkb.rs:10067:13

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb-protocol/src/protocol/xkb.rs:10067:13`
- last API call: `x11rb_protocol::protocol::xkb::GetNamesReply::serialize_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: `type_names` has an incorrect length
  left: 1
 right: 0
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
use x11rb_protocol::protocol::xkb::GetNamesReply;
use x11rb_protocol::protocol::xkb::GetNamesValueList;
use x11rb_protocol::protocol::xkb::SetOfGroup;
use x11rb_protocol::protocol::xkb::VMod;

fn main() {
    <x11rb_protocol::protocol::xkb::GetNamesReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetNamesReply { device_id: 0u8, sequence: 0u16, length: 0u32, min_key_code: 0u8, max_key_code: 0u8, n_types: 0u8, group_names: SetOfGroup::GROUP2, virtual_mods: VMod::M3, first_key: 0u8, n_keys: 0u8, indicators: 0u32, n_radio_groups: 0u8, n_key_aliases: 0u8, n_kt_levels: 0u16, value_list: GetNamesValueList { keycodes_name: None, geometry_name: None, symbols_name: None, phys_symbols_name: None, types_name: None, compat_name: None, type_names: Some(vec![0u32]), kt_level_names: None, indicator_names: None, virtual_mod_names: None, groups: None, key_names: None, key_aliases: None, radio_group_names: None } }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_protocol_src_protocol_xkb_rs_10067_13_25ad36`).

## Output

```
thread 'main' (323051) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:10067:13:
assertion `left == right` failed: `type_names` has an incorrect length
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <x11rb_protocol::protocol::xkb::GetNamesValueList>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:10067:13
   5: <x11rb_protocol::protocol::xkb::GetNamesReply as x11rb_protocol::x11_utils::Serialize>::serialize_into
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb-protocol/src/protocol/xkb.rs:10230:25
   6: repro_x11rb_protocol_src_protocol_xkb_rs_10067_13_25ad36::main
             at ./src/main.rs:7:5
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
