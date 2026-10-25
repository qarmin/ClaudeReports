# object/src/write/mod.rs:744:18

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:744:18`
- last API call: `object::write::Object::add_relocation`
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
use object::RelocationFlags;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Wasm, Architecture::LoongArch64, Endianness::default());
    let _ = object.add_relocation(Object::new(BinaryFormat::Coff, Architecture::LoongArch32, Endianness::Big).section_id(StandardSection::EhFrame), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::MachO, Architecture::Riscv32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Wasm { r_type: 0u8 } });
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_744_18_7dbd88`).

## Output

```
thread 'main' (1242667) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:744:18:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::Object>::add_relocation_internal
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:744:18
   4: <object::write::Object>::add_relocation
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:694:14
   5: repro_object_src_write_mod_rs_744_18_7dbd88::main
             at ./src/main.rs:11:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
