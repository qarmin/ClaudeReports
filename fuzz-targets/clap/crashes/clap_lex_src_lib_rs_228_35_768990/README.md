# clap_lex/src/lib.rs:228:35

- crate: https://github.com/clap-rs/clap @ ccc89b83875474399e9d9d70841163f6319d1a41
- kind: panic
- location: `clap_lex/src/lib.rs:228:35`
- last API call: `clap_lex::RawArgs::remaining`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
clap = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
clap_lex = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
```

`src/main.rs`:

```rust
use clap_lex::RawArgs;

fn main() {
    let raw_args = RawArgs::default();
    let raw_args2 = RawArgs::default();
    let mut arg_cursor = raw_args2.cursor();
    let _ = raw_args.next(&mut arg_cursor);
    let _ = raw_args2.remaining(&mut arg_cursor);
}
```

Run with `cargo run` in this directory (package `repro_clap_lex_src_lib_rs_228_35_768990`).

## Output

```
thread 'main' (459108) panicked at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_lex/src/lib.rs:228:35:
range start index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[std::ffi::os_str::OsString]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[std::ffi::os_str::OsString] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<std::ffi::os_str::OsString> as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <clap_lex::RawArgs>::remaining
             at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_lex/src/lib.rs:228:35
   7: repro_clap_lex_src_lib_rs_228_35_768990::main
             at ./src/main.rs:8:23
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
