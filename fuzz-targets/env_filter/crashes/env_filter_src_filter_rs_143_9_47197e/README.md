# env_filter/src/filter.rs:143:9

- crate: https://github.com/rust-cli/env_logger @ f80dca81223bb7cd50fc81f484e20eaef22be0aa
- kind: panic
- location: `env_filter/src/filter.rs:143:9`
- last API call: `env_filter::Builder::build`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to re-use consumed builder
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
env_filter = { git = "https://github.com/rust-cli/env_logger", rev = "f80dca81223bb7cd50fc81f484e20eaef22be0aa" }
```

`src/main.rs`:

```rust
use env_filter::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.build();
    let _ = builder.build();
}
```

Run with `cargo run` in this directory (package `repro_env_filter_src_filter_rs_143_9_47197e`).

## Output

```
thread 'main' (593545) panicked at /home/rafal/.cargo/git/checkouts/env_logger-45ea055fbda314b9/f80dca8/crates/env_filter/src/filter.rs:143:9:
attempt to re-use consumed builder
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <env_filter::filter::Builder>::build
             at /home/rafal/.cargo/git/checkouts/env_logger-45ea055fbda314b9/f80dca8/crates/env_filter/src/filter.rs:143:9
   3: repro_env_filter_src_filter_rs_143_9_47197e::main
             at ./src/main.rs:6:21
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
