# object/src/write/mod.rs:763:22

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:763:22`
- last API call: `object::write::Object::add_relocation`
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
use object::RelocationFlags;
use object::elf::R_TILEGX_DEST_IMM8_X1;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let object = Object::new(BinaryFormat::Elf, Architecture::Wasm32, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let mut object2 = Object::new(object.format(), Architecture::E2K32, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let _ = object2.add_relocation(Object::new(BinaryFormat::native_object(), Architecture::Arm, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::GnuProperty), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Wasm, Architecture::LoongArch32, Endianness::Big).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Elf { r_type: R_TILEGX_DEST_IMM8_X1 } });
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_763_22_7f8375`).

## Output

```
thread 'main' (1242535) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:763:22:
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
   6: <object::write::Object>::add_relocation_internal
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:763:22
   7: <object::write::Object>::add_relocation
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:694:14
   8: repro_object_src_write_mod_rs_763_22_7f8375::main
             at ./src/main.rs:13:21
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
