# object/src/write/mod.rs:979:9

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:979:9`
- last API call: `object::write::Object::append_section_data`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !self.is_bss()
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
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Hexagon, <object::write::macho::Encoder<object::endian::Endianness>>::new(Endianness::default(), false).endian());
    let section_id = object.section_id(StandardSection::UninitializedTls);
    let _ = object.append_section_data(section_id, &vec![], 0u64);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_979_9_7a0c2c`).

## Output

```
thread 'main' (1242868) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:979:9:
assertion failed: !self.is_bss()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::Section>::append_data
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:979:9
   4: <object::write::Object>::append_section_data
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:208:34
   5: repro_object_src_write_mod_rs_979_9_7a0c2c::main
             at ./src/main.rs:10:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
