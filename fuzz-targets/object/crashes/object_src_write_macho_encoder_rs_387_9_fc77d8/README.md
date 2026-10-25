# object/src/write/macho/encoder.rs:387:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/macho/encoder.rs:387:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/macho/encoder.rs#387
- last API call: `object::write::macho::Encoder<object::endian::Endianness>::load_command_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::endian::Endianness;

fn main() {
    let encoder = <object::write::macho::Encoder<object::endian::Endianness>>::new(Endianness::Big, false);
    let _ = encoder.load_command_size(u64::MAX);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_macho_encoder_rs_387_9_fc77d8`).

## Output

```
thread 'main' (2422644) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:387:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <object::write::macho::encoder::Encoder<object::endian::Endianness>>::load_command_size
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:387:9
   4: repro_object_src_write_macho_encoder_rs_387_9_fc77d8::main
             at ./src/main.rs:5:21
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
