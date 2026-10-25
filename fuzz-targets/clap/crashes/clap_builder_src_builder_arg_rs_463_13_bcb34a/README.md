# clap_builder/src/builder/arg.rs:463:13

- crate: https://github.com/clap-rs/clap @ ccc89b83875474399e9d9d70841163f6319d1a41
- kind: panic
- location: `clap_builder/src/builder/arg.rs:463:13`
- last API call: `clap_builder::Arg::visible_short_aliases`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
short alias name cannot be `-`
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
clap = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
clap_builder = { git = "https://github.com/clap-rs/clap", rev = "ccc89b83875474399e9d9d70841163f6319d1a41" }
```

`src/main.rs`:

```rust
use clap_builder::Arg;
use clap_builder::Id;

fn main() {
    let arg = Arg::new(Id::default());
    let _ = arg.clone().visible_short_aliases(vec!['-']);
}
```

Run with `cargo run` in this directory (package `repro_clap_builder_src_builder_arg_rs_463_13_bcb34a`).

## Output

```
thread 'main' (459265) panicked at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_builder/src/builder/arg.rs:463:13:
short alias name cannot be `-`
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <clap_builder::builder::arg::Arg>::visible_short_aliases::<alloc::vec::Vec<char>>
             at /home/rafal/.cargo/git/checkouts/clap-f7efb462f4319ddd/ccc89b8/clap_builder/src/builder/arg.rs:463:13
   3: repro_clap_builder_src_builder_arg_rs_463_13_bcb34a::main
             at ./src/main.rs:6:25
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
