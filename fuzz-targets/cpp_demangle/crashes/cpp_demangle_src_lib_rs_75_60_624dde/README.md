# cpp_demangle/src/lib.rs:75:60

- crate: https://github.com/gimli-rs/cpp_demangle @ 076632f924b1c87c7de8ef6a5c604edb97964978
- kind: panic
- location: `cpp_demangle/src/lib.rs:75:60`
- last API call: `cpp_demangle::ParseOptions::recursion_limit`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Recursion limit must be > 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cpp_demangle = { git = "https://github.com/gimli-rs/cpp_demangle", rev = "076632f924b1c87c7de8ef6a5c604edb97964978" }
```

`src/main.rs`:

```rust
use cpp_demangle::ParseOptions;

fn main() {
    let _ = ParseOptions::default().recursion_limit(0u32);
}
```

Run with `cargo run` in this directory (package `repro_cpp_demangle_src_lib_rs_75_60_624dde`).

## Output

```
thread 'main' (562260) panicked at /home/rafal/.cargo/git/checkouts/cpp_demangle-54464f5b14af48c9/076632f/src/lib.rs:75:60:
Recursion limit must be > 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<core::num::nonzero::NonZero<u32>>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <cpp_demangle::ParseOptions>::recursion_limit
             at /home/rafal/.cargo/git/checkouts/cpp_demangle-54464f5b14af48c9/076632f/src/lib.rs:75:60
   6: repro_cpp_demangle_src_lib_rs_75_60_624dde::main
             at ./src/main.rs:4:37
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
