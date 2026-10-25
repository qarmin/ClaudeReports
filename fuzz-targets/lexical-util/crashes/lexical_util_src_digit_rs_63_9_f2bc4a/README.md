# lexical-util/src/digit.rs:63:9

- crate: https://github.com/Alexhuszagh/rust-lexical @ afb2efa651f42f63415a3bf3d51ced6143132b2b
- kind: panic
- location: `lexical-util/src/digit.rs:63:9`
- last API call: `lexical_util::digit::digit_to_char_const`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lexical-util = { git = "https://github.com/Alexhuszagh/rust-lexical", rev = "afb2efa651f42f63415a3bf3d51ced6143132b2b", features = ["compact", "f128", "f16", "float16", "format", "lint", "parse-floats", "parse-integers", "power-of-two", "radix", "std", "write-floats", "write-integers"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = lexical_util::digit::digit_to_char_const(65535u32, 100u32);
}
```

Run with `cargo run` in this directory (package `repro_lexical_util_src_digit_rs_63_9_f2bc4a`).

## Output

```
thread 'main' (468030) panicked at /home/rafal/.cargo/git/checkouts/rust-lexical-b54ef1c212894bbd/afb2efa/lexical-util/src/digit.rs:63:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: lexical_util::digit::digit_to_char_const
             at /home/rafal/.cargo/git/checkouts/rust-lexical-b54ef1c212894bbd/afb2efa/lexical-util/src/digit.rs:63:9
   4: repro_lexical_util_src_digit_rs_63_9_f2bc4a::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
