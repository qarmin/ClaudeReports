# fluent-syntax/src/unicode.rs:93:31

- crate: https://github.com/projectfluent/fluent-rs @ 0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda
- kind: panic
- location: `fluent-syntax/src/unicode.rs:93:31`
- last API call: `fluent_syntax::unicode::unescape_unicode_to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
start byte index 2 is not a char boundary; it is inside '\u{62599}' (bytes 1..5 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fluent = { git = "https://github.com/projectfluent/fluent-rs", rev = "0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda" }
fluent-syntax = { git = "https://github.com/projectfluent/fluent-rs", rev = "0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = fluent_syntax::unicode::unescape_unicode_to_string("\\\u{62599}\\");
}
```

Run with `cargo run` in this directory (package `repro_fluent_syntax_src_unicode_rs_93_31_237043`).

## Output

```
thread 'main' (450136) panicked at /home/rafal/.cargo/git/checkouts/fluent-rs-4af0e84a8ba1cce1/0f87afb/fluent-syntax/src/unicode.rs:93:31:
start byte index 2 is not a char boundary; it is inside '\u{62599}' (bytes 1..5 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: fluent_syntax::unicode::unescape::<alloc::string::String>
             at /home/rafal/.cargo/git/checkouts/fluent-rs-4af0e84a8ba1cce1/0f87afb/fluent-syntax/src/unicode.rs:93:31
   8: fluent_syntax::unicode::unescape_unicode_to_string
             at /home/rafal/.cargo/git/checkouts/fluent-rs-4af0e84a8ba1cce1/0f87afb/fluent-syntax/src/unicode.rs:138:17
   9: repro_fluent_syntax_src_unicode_rs_93_31_237043::main
             at ./src/main.rs:2:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
