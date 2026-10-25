# object/src/write/macho/encoder.rs:953:13

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/macho/encoder.rs:953:13`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/macho/encoder.rs#953
- last API call: `object::write::macho::CodeSignatureEncoder::code_directory`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: cd.version >= macho::CS_SUPPORTSCODELIMIT64
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::macho::CS_EXECSEG_SKIP_LV;
use object::macho::CS_RESTRICT;
use object::macho::CS_SUPPORTSSCATTER;
use object::write::macho::CodeDirectory;
use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    code_signature_encoder.code_directory::<Vec<u8>>(&mut vec![], &CodeDirectory { length: 0u32, version: CS_SUPPORTSSCATTER, flags: CS_RESTRICT, hash_offset: 0u32, ident_offset: 0u32, n_special_slots: 0u32, n_code_slots: 0u32, code_limit: u64::MAX, hash_size: 255u8, hash_type: <object::macho::CsHashType as object::Wrap>::from_inner(72u8), platform: 1u8, page_size: 191u8, scatter_offset: 4077522095u32, team_offset: 32u32, exec_seg_base: 0u64, exec_seg_limit: 4294967295u64, exec_seg_flags: CS_EXECSEG_SKIP_LV });
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_macho_encoder_rs_953_13_573f08`).

## Output

```
thread 'main' (2423954) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:953:13:
assertion failed: cd.version >= macho::CS_SUPPORTSCODELIMIT64
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::macho::encoder::CodeSignatureEncoder>::code_directory::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:953:13
   4: repro_object_src_write_macho_encoder_rs_953_13_573f08::main
             at ./src/main.rs:9:28
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
