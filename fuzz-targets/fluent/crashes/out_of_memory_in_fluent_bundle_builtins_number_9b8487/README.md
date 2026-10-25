# out of memory in fluent_bundle::builtins::NUMBER

- crate: https://github.com/projectfluent/fluent-rs @ 0f87afbbc3fe7d20c61a3fc75cb5d0e123aefdda
- kind: out of memory
- last API call: `fluent_bundle::builtins::NUMBER`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling fluent_bundle::builtins::NUMBER
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
use fluent_bundle::builtins::NUMBER;

fn main() {
    let vec = vec![];
    let fluent_args = FluentArgs::with_capacity(1000000000usize);
    let _ = NUMBER(&vec, &fluent_args);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_fluent_bundle_builtins_number_9b8487`).

## Output

```
memory allocation of 144000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<(alloc::borrow::Cow<str>, fluent_bundle::types::FluentValue)>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <fluent_bundle::args::FluentArgs>::with_capacity
             at /home/rafal/.cargo/git/checkouts/fluent-rs-4af0e84a8ba1cce1/0f87afb/fluent-bundle/src/args.rs:66:14
  10: repro_out_of_memory_in_fluent_bundle_builtins_number_9b8487::main
             at ./src/main.rs:6:23
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
