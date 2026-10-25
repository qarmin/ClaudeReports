# anstyle-parse/src/lib.rs:343:9

- crate: https://github.com/rust-cli/anstyle @ d891cdae9f5d9e923232bb42cecc597be09afe7a
- kind: panic
- location: `anstyle-parse/src/lib.rs:343:9`
- last API call: `anstyle_parse::AsciiParser::add`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code: multi-byte UTF8 characters are unsupported
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
anstyle = { git = "https://github.com/rust-cli/anstyle", rev = "d891cdae9f5d9e923232bb42cecc597be09afe7a" }
anstyle-parse = { git = "https://github.com/rust-cli/anstyle", rev = "d891cdae9f5d9e923232bb42cecc597be09afe7a" }
```

`src/main.rs`:

```rust
use anstyle_parse::AsciiParser;

fn main() {
    let mut ascii_parser = AsciiParser;
    let _ = <anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator>::add(&mut ascii_parser, 0u8);
}
```

Run with `cargo run` in this directory (package `repro_anstyle_parse_src_lib_rs_343_9_5e41fd`).

## Output

```
thread 'main' (610206) panicked at /home/rafal/.cargo/git/checkouts/anstyle-ceff55ea1eef6a73/d891cda/crates/anstyle-parse/src/lib.rs:343:9:
internal error: entered unreachable code: multi-byte UTF8 characters are unsupported
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator>::add
             at /home/rafal/.cargo/git/checkouts/anstyle-ceff55ea1eef6a73/d891cda/crates/anstyle-parse/src/lib.rs:343:9
   3: repro_anstyle_parse_src_lib_rs_343_9_5e41fd::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
