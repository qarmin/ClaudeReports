# rust/src/moparser.rs:83:52

- crate: https://github.com/mondeja/rspolib @ 12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8
- kind: panic
- location: `rust/src/moparser.rs:83:52`
- last API call: `rspolib::mofile`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rspolib = { git = "https://github.com/mondeja/rspolib", rev = "12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8" }
```

`src/main.rs`:

```rust
use rspolib::FileOptions;

fn main() {
    let _ = rspolib::mofile::<rspolib::FileOptions>(FileOptions { path_or_content: "".to_string(), wrapwidth: 0usize, byte_content: None });
}
```

Run with `cargo run` in this directory (package `repro_rust_src_moparser_rs_83_52_4c27dd`).

## Output

```
thread 'main' (1364781) panicked at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/moparser.rs:83:52:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&mut alloc::vec::Vec<u8>>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <rspolib::moparser::MOFileParser>::new
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/moparser.rs:83:52
   6: rspolib::file::mofile::mofile::<rspolib::file::FileOptions>
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/file/mofile.rs:71:22
   7: repro_rust_src_moparser_rs_83_52_4c27dd::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
