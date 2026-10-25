# indicatif/src/multi.rs:262:62

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: panic
- location: `indicatif/src/multi.rs:262:62`
- last API call: `indicatif::MultiProgress::insert_after`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
indicatif = { git = "https://github.com/console-rs/indicatif", rev = "6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9" }
```

`src/main.rs`:

```rust
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;

fn main() {
    let multi_progress = MultiProgress::default();
    let _ = multi_progress.insert_after(&ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(0u8)), ProgressBar::new_spinner());
}
```

Run with `cargo run` in this directory (package `repro_indicatif_src_multi_rs_262_62_e941db`).

## Output

```
thread 'main' (1268920) panicked at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/multi.rs:262:62:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<indicatif::multi::MultiStateIndex>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <indicatif::multi::MultiProgress>::insert_after
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/multi.rs:262:62
   6: repro_indicatif_src_multi_rs_262_62_e941db::main
             at ./src/main.rs:7:28
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
