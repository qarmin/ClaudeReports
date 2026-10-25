# toml_parser-1.1.5+spec/src/source.rs:252:20

- crate: toml 1.1.8+spec-1.1.0
- kind: panic
- location: `toml_parser-1.1.5+spec/src/source.rs:252:20`
- last API call: `toml_parser::Span::add<usize>`
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
toml = { version = "=1.1.8+spec-1.1.0" }
toml_parser = { version = "=1.1.5+spec-1.1.0" }
```

`src/main.rs`:

```rust
use toml_parser::Span;

fn main() {
    let _ = <toml_parser::Span as ::core::ops::Add<usize>>::add(Span::new_unchecked(10000000000000000000usize, 4294967295usize), 18446744073709551614usize);
}
```

Run with `cargo run` in this directory (package `repro_toml_parser_1_1_5_spec_src_source_rs_252_20_7bec53`).

## Output

```
thread 'main' (374378) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_parser-1.1.5+spec-1.1.0/src/source.rs:252:20:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <toml_parser::source::Span as core::ops::arith::Add<usize>>::add
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_parser-1.1.5+spec-1.1.0/src/source.rs:252:20
   4: repro_toml_parser_1_1_5_spec_src_source_rs_252_20_7bec53::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
