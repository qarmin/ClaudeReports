# object/src/write/elf/encoder.rs:1227:15

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/elf/encoder.rs:1227:15`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/elf/encoder.rs#1227
- last API call: `object::write::elf::Encoder<object::endian::Endianness>::gnu_verdef_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::build::elf::Builder;
use object::endian::Endianness;

fn main() {
    let builder = Builder::new(Endianness::Little, false);
    let encoder = builder.encoder();
    let _ = encoder.gnu_verdef_size(0u16, 10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_elf_encoder_rs_1227_15_ace09e`).

## Output

```
thread 'main' (2423326) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/elf/encoder.rs:1227:15:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <object::write::elf::encoder::Encoder<object::endian::Endianness>>::gnu_verdef_size
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/elf/encoder.rs:1227:15
   4: repro_object_src_write_elf_encoder_rs_1227_15_ace09e::main
             at ./src/main.rs:7:21
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
