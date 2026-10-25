# clang-sys/src/support.rs:189:45

- crate: https://github.com/rust-lang/rust-bindgen @ f08d4fff0e060fb4081222b0ad624e0bd7ec247d
- kind: panic
- location: `clang-sys/src/support.rs:189:45`
- source: https://docs.rs/crate/clang-sys/1.9.1/source/src/support.rs#189
- last API call: `bindgen::Builder::generate`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: "could not run executable `/usr/lib/llvm-21/bin/clang`: nul byte found in provided data"
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bindgen = { git = "https://github.com/rust-lang/rust-bindgen", rev = "f08d4fff0e060fb4081222b0ad624e0bd7ec247d" }
```

`src/main.rs`:

```rust
use bindgen::Builder;

fn main() {
    let builder = Builder::default();
    let builder2 = builder.clone().header::<String>("".to_string());
    let builder3 = builder2.clone().clang_arg::<String>("\0".to_string());
    let _ = builder3.clone().generate();
}
```

Run with `cargo run` in this directory (package `repro_clang_sys_src_support_rs_189_45_726c49`).

## Output

```
thread 'main' (528489) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clang-sys-1.9.1/src/support.rs:189:45:
called `Result::unwrap()` on an `Err` value: "could not run executable `/usr/lib/llvm-21/bin/clang`: nul byte found in provided data"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(alloc::string::String, alloc::string::String), alloc::string::String>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: clang_sys::support::run_clang
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clang-sys-1.9.1/src/support.rs:189:45
   5: clang_sys::support::parse_search_paths
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clang-sys-1.9.1/src/support.rs:227:18
   6: <clang_sys::support::Clang>::new::<std::path::PathBuf>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clang-sys-1.9.1/src/support.rs:39:29
   7: <clang_sys::support::Clang>::find
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clang-sys-1.9.1/src/support.rs:124:29
   8: <bindgen::Bindings>::generate::detect_include_paths
             at /home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:858:31
   9: <bindgen::Bindings>::generate
             at /home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:890:9
  10: <bindgen::Builder>::generate
             at /home/rafal/.cargo/git/checkouts/rust-bindgen-d3080e0c1112964e/f08d4ff/bindgen/lib.rs:380:9
  11: repro_clang_sys_src_support_rs_189_45_726c49::main
             at ./src/main.rs:7:30
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
