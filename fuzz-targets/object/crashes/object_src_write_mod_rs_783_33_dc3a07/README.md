# object/src/write/mod.rs:783:33

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:783:33`
- last API call: `object::write::Object::add_relocation_with_subtractor`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
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
use object::RelocationEncoding;
use object::RelocationFlags;
use object::RelocationKind;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Sparc, Endianness::Little);
    let _ = object.add_relocation_with_subtractor(Object::new(BinaryFormat::Elf, Architecture::E2K32, Endianness::default()).section_id(StandardSection::Data), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::native_object(), Architecture::PowerPc64, Endianness::Little).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Generic { kind: RelocationKind::Relative, encoding: RelocationEncoding::E2KDisp, size: 1u8 } }, None);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_783_33_dc3a07`).

## Output

```
thread 'main' (1242402) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:783:33:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[object::write::Section]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:245:14
   4: <[object::write::Section] as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<object::write::Section> as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <object::write::Object>::write_relocation_addend
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:783:33
   7: <object::write::Object>::add_relocation_internal
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:760:18
   8: <object::write::Object>::add_relocation_with_subtractor
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:719:14
   9: repro_object_src_write_mod_rs_783_33_dc3a07::main
             at ./src/main.rs:13:20
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
