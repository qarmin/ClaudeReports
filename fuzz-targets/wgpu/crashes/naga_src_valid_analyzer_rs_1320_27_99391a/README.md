# naga/src/valid/analyzer.rs:1320:27

- crate: https://github.com/gfx-rs/wgpu @ efcf445784dd31537f09bc61bad27431de713fce
- kind: panic
- location: `naga/src/valid/analyzer.rs:1320:27`
- last API call: `naga::valid::ModuleInfo::get_entry_point`
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
wgpu = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
naga = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
```

`src/main.rs`:

```rust
use naga::valid::ModuleInfo;

fn main() {
    let module_info = ModuleInfo::default();
    let _ = module_info.get_entry_point(0usize);
}
```

Run with `cargo run` in this directory (package `repro_naga_src_valid_analyzer_rs_1320_27_99391a`).

## Output

```
thread 'main' (2029098) panicked at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/naga/src/valid/analyzer.rs:1320:27:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[naga::valid::analyzer::FunctionInfo]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[naga::valid::analyzer::FunctionInfo] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<naga::valid::analyzer::FunctionInfo> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <naga::valid::ModuleInfo>::get_entry_point
             at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/naga/src/valid/analyzer.rs:1320:27
   7: repro_naga_src_valid_analyzer_rs_1320_27_99391a::main
             at ./src/main.rs:5:25
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
