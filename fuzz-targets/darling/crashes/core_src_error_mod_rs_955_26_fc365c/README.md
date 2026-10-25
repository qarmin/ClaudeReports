# core/src/error/mod.rs:955:26

- crate: https://github.com/TedDriggs/darling @ 70b280dede2eaa1ee53f445a970657b16299767b
- kind: panic
- location: `core/src/error/mod.rs:955:26`
- last API call: `darling_core::error::Accumulator::default`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
darling::error::Accumulator dropped without being finished
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
darling = { git = "https://github.com/TedDriggs/darling", rev = "70b280dede2eaa1ee53f445a970657b16299767b" }
darling_core = { git = "https://github.com/TedDriggs/darling", rev = "70b280dede2eaa1ee53f445a970657b16299767b" }
```

`src/main.rs`:

```rust
use darling_core::error::Accumulator;

fn main() {
    let _ = Accumulator::default();
}
```

Run with `cargo run` in this directory (package `repro_core_src_error_mod_rs_955_26_fc365c`).

## Output

```
thread 'main' (1446566) panicked at /home/rafal/.cargo/git/checkouts/darling-9bac648dba2757b7/70b280d/core/src/error/mod.rs:955:26:
darling::error::Accumulator dropped without being finished
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <darling_core::error::Accumulator as core::ops::drop::Drop>::drop
             at /home/rafal/.cargo/git/checkouts/darling-9bac648dba2757b7/70b280d/core/src/error/mod.rs:955:26
   3: core::ptr::drop_glue::<darling_core::error::Accumulator>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   4: repro_core_src_error_mod_rs_955_26_fc365c::main
             at ./src/main.rs:4:35
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
