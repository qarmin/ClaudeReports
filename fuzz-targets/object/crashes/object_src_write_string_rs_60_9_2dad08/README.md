# object/src/write/string.rs:60:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/string.rs:60:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/string.rs#60
- last API call: `object::write::Object::write`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !string.contains(&0)
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
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Coff, Architecture::M68k, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let _ = object.add_subsection(StandardSection::TlsVariables, &vec![80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    let _ = object.write();
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_string_rs_60_9_2dad08`).

## Output

```
thread 'main' (2424713) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/string.rs:60:9:
assertion failed: !string.contains(&0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::string::StringTable>::add
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/string.rs:60:9
   4: <object::write::coff::writer::Writer>::add_string
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/coff/writer.rs:430:21
   5: <object::write::coff::writer::Writer>::add_name
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/coff/writer.rs:438:29
   6: <object::write::Object>::coff_write
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/coff/object.rs:435:50
   7: <object::write::Object>::emit
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/mod.rs:825:40
   8: <object::write::Object>::write
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/mod.rs:800:14
   9: repro_object_src_write_string_rs_60_9_2dad08::main
             at ./src/main.rs:9:20
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
