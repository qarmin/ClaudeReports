# krilla/src/interchange/tagging/fmt.rs:65:9

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interchange/tagging/fmt.rs:65:9`
- last API call: `krilla::tagging::fmt::Indent::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Formatting argument out of range
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::tagging::fmt::Indent;

fn main() {
    let indent = Indent(1000000000000000000usize);
    let _ = indent.to_string();
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interchange_tagging_fmt_rs_65_9_906043`).

## Output

```
thread 'main' (250139) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interchange/tagging/fmt.rs:65:9:
Formatting argument out of range
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <core::fmt::rt::Argument>::from_usize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/rt.rs:132:13
   3: <krilla::interchange::tagging::fmt::Indent as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interchange/tagging/fmt.rs:65:9
   4: <krilla::interchange::tagging::fmt::Indent as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   5: <krilla::interchange::tagging::fmt::Indent as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   6: repro_krilla_src_interchange_tagging_fmt_rs_65_9_906043::main
             at ./src/main.rs:5:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
