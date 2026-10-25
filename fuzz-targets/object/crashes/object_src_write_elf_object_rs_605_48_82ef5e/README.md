# object/src/write/elf/object.rs:605:48

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/elf/object.rs:605:48`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/elf/object.rs#605
- last API call: `object::write::Object::add_elf_gnu_property_u32`
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
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;

fn main() {
    let binary_format = BinaryFormat::native_object();
    let mut object = Object::new(binary_format, Architecture::Unknown, Endianness::Big);
    object.add_elf_gnu_property_u32(<object::elf::GnuPropertyType as object::Wrap>::from_inner(0u32), 0u32);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_elf_object_rs_605_48_82ef5e`).

## Output

```
thread 'main' (2423660) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/elf/object.rs:605:48:
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
   4: <core::option::Option<object::common::AddressSize>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <object::write::Object>::elf_is_64
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/elf/object.rs:605:48
   6: <object::write::Object>::add_elf_gnu_property_u32
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/elf/object.rs:41:36
   7: repro_object_src_write_elf_object_rs_605_48_82ef5e::main
             at ./src/main.rs:9:12
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
