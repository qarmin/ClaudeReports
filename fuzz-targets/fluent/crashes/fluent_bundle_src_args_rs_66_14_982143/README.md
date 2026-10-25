# fluent-bundle/src/args.rs:66:14

- crate: https://github.com/projectfluent/fluent-rs @ 0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda
- kind: panic
- location: `fluent-bundle/src/args.rs:66:14`
- last API call: `fluent_bundle::FluentArgs::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fluent = { git = "https://github.com/projectfluent/fluent-rs", rev = "0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda" }
fluent-bundle = { git = "https://github.com/projectfluent/fluent-rs", rev = "0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda" }
```

`src/main.rs`:

```rust
use fluent_bundle::FluentArgs;

fn main() {
    let _ = FluentArgs::with_capacity(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_fluent_bundle_src_args_rs_66_14_982143`).

## Output

```
thread 'main' (449957) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   5: <alloc::raw_vec::RawVec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <fluent_bundle::args::FluentArgs>::with_capacity
             at /home/rafal/.cargo/git/checkouts/fluent-rs-4af0e84a8ba1cce1/0f87afb/fluent-bundle/src/args.rs:66:14
   9: repro_fluent_bundle_src_args_rs_66_14_982143::main
             at ./src/main.rs:4:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
