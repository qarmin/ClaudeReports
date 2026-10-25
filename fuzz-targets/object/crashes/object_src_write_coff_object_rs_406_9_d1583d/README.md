# object/src/write/coff/object.rs:406:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/coff/object.rs:406:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/coff/object.rs#406
- last API call: `object::write::Object::add_coff_exports`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: Pe
 right: Coff
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
use object::write::CoffExportStyle;
use object::write::Object;

fn main() {
    let mut object = Object::new(BinaryFormat::Pe, Architecture::PowerPc, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    object.add_coff_exports(CoffExportStyle::Gnu);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_coff_object_rs_406_9_d1583d`).

## Output

```
thread 'main' (2423383) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/coff/object.rs:406:9:
assertion `left == right` failed
  left: Pe
 right: Coff
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<object::common::BinaryFormat, object::common::BinaryFormat>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <object::write::Object>::add_coff_exports
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/coff/object.rs:406:9
   5: repro_object_src_write_coff_object_rs_406_9_d1583d::main
             at ./src/main.rs:8:12
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
