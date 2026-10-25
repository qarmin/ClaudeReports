# object/src/write/macho/encoder.rs:929:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/macho/encoder.rs:929:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/macho/encoder.rs#929
- last API call: `object::write::macho::CodeSignatureEncoder::code_directory_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: version <= macho::CS_SUPPORTSEXECSEG
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    let _ = code_signature_encoder.code_directory_size(<object::macho::CsVersion as object::Wrap>::from_inner(1000000000u32));
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_macho_encoder_rs_929_9_d8b45e`).

## Output

```
thread 'main' (2423490) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:929:9:
assertion failed: version <= macho::CS_SUPPORTSEXECSEG
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::macho::encoder::CodeSignatureEncoder>::code_directory_size
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:929:9
   4: repro_object_src_write_macho_encoder_rs_929_9_d8b45e::main
             at ./src/main.rs:5:36
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
