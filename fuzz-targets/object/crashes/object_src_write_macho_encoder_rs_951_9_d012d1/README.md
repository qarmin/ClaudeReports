# object/src/write/macho/encoder.rs:951:9

- crate: object 0.40.0
- kind: panic
- location: `object/src/write/macho/encoder.rs:951:9`
- source: https://docs.rs/crate/object/0.40.0/source/src/write/macho/encoder.rs#951
- last API call: `object::write::macho::CodeSignatureEncoder::code_directory`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: cd.version <= macho::CS_SUPPORTSEXECSEG
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
object = { version = "=0.40.0", features = ["all", "alloc", "archive", "build", "build_core", "coff", "compression", "core", "doc", "elf", "macho", "names", "pe", "read", "read_core", "rustc-dep-of-std", "std", "unaligned", "unstable", "unstable-all", "wasm", "write", "write_core", "write_std", "xcoff"] }
```

`src/main.rs`:

```rust
use object::macho::CS_EXECSEG_MAIN_BINARY;
use object::macho::CS_EXEC_SET_KILL;
use object::macho::CS_HASHTYPE_SHA384;
use object::macho::CsVersion;
use object::write::macho::CodeDirectory;
use object::write::macho::CodeSignatureEncoder;

fn main() {
    let code_signature_encoder = CodeSignatureEncoder;
    code_signature_encoder.code_directory::<Vec<u8>>(&mut vec![], &CodeDirectory { length: 0u32, version: CsVersion(1000000000u32), flags: CS_EXEC_SET_KILL, hash_offset: 0u32, ident_offset: 0u32, n_special_slots: 0u32, n_code_slots: 0u32, code_limit: 0u64, hash_size: 0u8, hash_type: CS_HASHTYPE_SHA384, platform: 0u8, page_size: 0u8, scatter_offset: 0u32, team_offset: 0u32, exec_seg_base: 0u64, exec_seg_limit: 0u64, exec_seg_flags: CS_EXECSEG_MAIN_BINARY });
}
```

Run with `cargo run` in this directory (package `repro_object_src_write_macho_encoder_rs_951_9_d012d1`).

## Output

```
thread 'main' (2422813) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:951:9:
assertion failed: cd.version <= macho::CS_SUPPORTSEXECSEG
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <object::write::macho::encoder::CodeSignatureEncoder>::code_directory::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/write/macho/encoder.rs:951:9
   4: repro_object_src_write_macho_encoder_rs_951_9_d012d1::main
             at ./src/main.rs:10:28
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
