# rawler/src/cfa.rs:124:12

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/cfa.rs:124:12`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/cfa.rs#124
- last API call: `rawler::cfa::CFA::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Unknown CFA size "a"
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::cfa::CFA;

fn main() {
    let _ = CFA::new("a");
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_cfa_rs_124_12_a0c181`).

## Output

```
thread 'main' (431280) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/cfa.rs:124:12:
Unknown CFA size "a"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <rawler::cfa::CFA>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/cfa.rs:124:12
   3: repro_rawler_src_cfa_rs_124_12_a0c181::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
