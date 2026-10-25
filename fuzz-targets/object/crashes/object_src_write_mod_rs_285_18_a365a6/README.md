# object/src/write/mod.rs:285:18

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:285:18`
- last API call: `object::write::Object::section_id`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { git = "https://github.com/gimli-rs/object", rev = "4ef27e429dd86949ac3d3a995b3266440bedcf5c", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let _ = Object::new(BinaryFormat::Wasm, Architecture::Wasm32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_285_18_a365a6`).

## Output

```
thread 'main' (1241961) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:285:18:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::Object>::section_info
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:285:18
   4: <object::write::Object>::section_id::{closure#0}
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:227:57
   5: <core::option::Option<object::write::SectionId>>::unwrap_or_else::<<object::write::Object>::section_id::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:21
   6: <object::write::Object>::section_id
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:226:14
   7: repro_object_src_write_mod_rs_285_18_a365a6::main
             at ./src/main.rs:7:159
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
