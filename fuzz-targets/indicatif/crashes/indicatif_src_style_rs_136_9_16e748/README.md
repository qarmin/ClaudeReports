# indicatif/src/style.rs:136:9

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: panic
- location: `indicatif/src/style.rs:136:9`
- last API call: `indicatif::ProgressStyle::progress_chars`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
at least 2 progress chars required
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
indicatif = { git = "https://github.com/console-rs/indicatif", rev = "6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9" }
```

`src/main.rs`:

```rust
use indicatif::HumanFloatCount;
use indicatif::ProgressStyle;

fn main() {
    let human_float_count = HumanFloatCount(1.0f64);
    let string = human_float_count.to_string();
    let _ = ProgressStyle::with_template(&string).unwrap().progress_chars("");
}
```

Run with `cargo run` in this directory (package `repro_indicatif_src_style_rs_136_9_16e748`).

## Output

```
thread 'main' (1269118) panicked at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:136:9:
at least 2 progress chars required
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <indicatif::style::ProgressStyle>::progress_chars
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:136:9
   3: repro_indicatif_src_style_rs_136_9_16e748::main
             at ./src/main.rs:7:60
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
