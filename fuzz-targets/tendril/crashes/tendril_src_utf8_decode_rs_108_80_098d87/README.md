# tendril/src/utf8_decode.rs:108:80

- crate: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45
- kind: panic
- location: `tendril/src/utf8_decode.rs:108:80`
- last API call: `tendril::IncompleteUtf8::try_to_complete_codepoint`
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
tendril = { git = "https://github.com/servo/html5ever", rev = "7760920edff08e6dfd4b62affee39d8084e9dc45" }
```

`src/main.rs`:

```rust
use tendril::IncompleteUtf8;

fn main() {
    let mut incomplete_utf8 = IncompleteUtf8 { buffer: [0u8, 0u8, 255u8, 0u8], buffer_len: 3u8 };
    let _ = incomplete_utf8.try_to_complete_codepoint(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_tendril_src_utf8_decode_rs_108_80_098d87`).

## Output

```
thread 'main' (358123) panicked at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:108:80:
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
   4: <core::option::Option<usize>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <tendril::utf8_decode::IncompleteUtf8>::try_complete_offsets
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:108:80
   6: <tendril::utf8_decode::IncompleteUtf8>::try_to_complete_codepoint
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:141:50
   7: repro_tendril_src_utf8_decode_rs_108_80_098d87::main
             at ./src/main.rs:5:29
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
