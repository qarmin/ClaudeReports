# naga/src/arena/range.rs:132:9

- crate: wgpu 30.0.1
- kind: panic
- location: `naga/src/arena/range.rs:132:9`
- source: https://docs.rs/crate/naga/30.0.1/source/src/arena/range.rs#132
- last API call: `naga::Arena<naga::diagnostic_filter::DiagnosticFilterNode>::range_from`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: inner.start <= inner.end
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
fn main() {
    let arena = <naga::Arena<naga::diagnostic_filter::DiagnosticFilterNode>>::new();
    let _ = arena.range_from(1usize);
}
```

Run with `cargo run` in this directory (package `repro_naga_src_arena_range_rs_132_9_669e2c`).

## Output

```
thread 'main' (472132) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-30.0.1/src/arena/range.rs:132:9:
assertion failed: inner.start <= inner.end
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <naga::arena::range::Range<naga::diagnostic_filter::DiagnosticFilterNode>>::from_index_range
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-30.0.1/src/arena/range.rs:132:9
   4: <naga::arena::Arena<naga::diagnostic_filter::DiagnosticFilterNode>>::range_from
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-30.0.1/src/arena/mod.rs:191:9
   5: repro_naga_src_arena_range_rs_132_9_669e2c::main
             at ./src/main.rs:3:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
