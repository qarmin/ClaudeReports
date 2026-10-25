# naga/src/proc/type_methods.rs:460:79

- crate: wgpu 30.0.1
- kind: panic
- location: `naga/src/proc/type_methods.rs:460:79`
- source: https://docs.rs/crate/naga/30.0.1/source/src/proc/type_methods.rs#460
- last API call: `naga::ir::TypeInner::component_type`
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
wgpu = { version = "=30.0.1" }
naga = { version = "=30.0.1" }
```

`src/main.rs`:

```rust
use naga::ir::TypeInner;

fn main() {
    let _ = TypeInner::Struct { members: vec![], span: 1u32 }.component_type(0usize);
}
```

Run with `cargo run` in this directory (package `repro_naga_src_proc_type_methods_rs_460_79_3ca6e8`).

## Output

```
thread 'main' (472251) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-30.0.1/src/proc/type_methods.rs:460:79:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[naga::ir::StructMember]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[naga::ir::StructMember] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<naga::ir::StructMember> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <naga::ir::TypeInner>::component_type
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-30.0.1/src/proc/type_methods.rs:460:79
   7: repro_naga_src_proc_type_methods_rs_460_79_3ca6e8::main
             at ./src/main.rs:4:63
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
