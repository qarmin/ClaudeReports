# object/src/write/macho/object.rs:435:16

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/macho/object.rs:435:16`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/macho/object.rs#435
- last API call: `object::write::Object::add_relocation`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
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
use object::RelocationFlags;
use object::endian::Endianness;
use object::macho::PPC_RELOC_HI16;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let object = Object::new(BinaryFormat::MachO, Architecture::SuperH, Endianness::Little);
    let mut object2 = Object::new(object.format(), Architecture::Xtensa, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let _ = object2.add_relocation(Object::new(BinaryFormat::Elf, Architecture::SuperH, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Pe, Architecture::Msp430, Endianness::default()).add_file_symbol(vec![]), addend: 1i64, flags: RelocationFlags::MachO { r_type: PPC_RELOC_HI16, r_pcrel: false, r_length: 10u8 } });
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_macho_object_rs_435_16_383d6f`).

## Output

```
thread 'main' (2425692) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/object.rs:435:16:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <object::write::Object>::macho_relocation_size
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/object.rs:435:16
   4: <object::write::Object>::write_relocation_addend
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/mod.rs:764:41
   5: <object::write::Object>::add_relocation_internal
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/mod.rs:746:18
   6: <object::write::Object>::add_relocation
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/mod.rs:684:14
   7: repro_object_src_write_macho_object_rs_435_16_383d6f::main
             at ./src/main.rs:13:21
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
