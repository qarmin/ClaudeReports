# built/src/environment.rs:55:33

- crate: https://github.com/lukaslueg/built @ 4ba8f7b0e4ceddecce72a0d170131dcd0a6497dc
- kind: panic
- location: `built/src/environment.rs:55:33`
- last API call: `built::util::detect_ci`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
no entry found for key
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
built = { git = "https://github.com/lukaslueg/built", rev = "4ba8f7b0e4ceddecce72a0d170131dcd0a6497dc" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = built::util::detect_ci();
}
```

Run with `cargo run` in this directory (package `repro_built_src_environment_rs_55_33_ce8500`).

## Output

```
thread 'main' (608812) panicked at /home/rafal/.cargo/git/checkouts/built-ecf7d7ca1e87250f/4ba8f7b/src/environment.rs:55:33:
no entry found for key
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<&(alloc::string::String, core::cell::RefCell<built::environment::EnvironmentValue>)>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <std::collections::hash::map::HashMap<alloc::string::String, (alloc::string::String, core::cell::RefCell<built::environment::EnvironmentValue>)> as core::ops::index::Index<&str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:1516:23
   6: <built::environment::EnvironmentMap>::new
             at /home/rafal/.cargo/git/checkouts/built-ecf7d7ca1e87250f/4ba8f7b/src/environment.rs:55:33
   7: built::util::detect_ci
             at /home/rafal/.cargo/git/checkouts/built-ecf7d7ca1e87250f/4ba8f7b/src/util.rs:61:5
   8: repro_built_src_environment_rs_55_33_ce8500::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
