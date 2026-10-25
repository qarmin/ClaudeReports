# rawler/src/devtools/mod.rs:39:49

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/devtools/mod.rs:39:49`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/devtools/mod.rs#39
- last API call: `rawler::devtools::dump_buf`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Unable to create file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
fn main() {
    rawler::devtools::dump_buf::<Vec<u8>>("", vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_devtools_mod_rs_39_49_0ed717`).

## Output

```
thread 'main' (431590) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/devtools/mod.rs:39:49:
Unable to create file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<std::fs::File, core::io::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: rawler::devtools::dump_buf::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/devtools/mod.rs:39:49
   5: repro_rawler_src_devtools_mod_rs_39_49_0ed717::main
             at ./src/main.rs:2:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
