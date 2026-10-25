# object/src/write/mod.rs:340:18

- crate: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c
- kind: panic
- location: `object/src/write/mod.rs:340:18`
- last API call: `object::write::Object::add_subsection`
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
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let _ = Object::new(BinaryFormat::Xcoff, Architecture::Wasm32, Endianness::Little).add_subsection(StandardSection::UninitializedData, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_mod_rs_340_18_bfa265`).

## Output

```
thread 'main' (1241705) panicked at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:340:18:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::Object>::subsection_name
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:340:18
   4: <object::write::Object>::subsection_info
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:328:25
   5: <object::write::Object>::add_subsection
             at /home/rafal/.cargo/git/checkouts/object-7f5857c5a53fbfc8/4ef27e4/src/write/mod.rs:298:53
   6: repro_object_src_write_mod_rs_340_18_bfa265::main
             at ./src/main.rs:8:88
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
