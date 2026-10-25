# indicatif/src/multi.rs:276:17

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: panic
- location: `indicatif/src/multi.rs:276:17`
- last API call: `indicatif::MultiProgress::remove`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: Arc::ptr_eq(&self.state, state)
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
    let progress_draw_target = ProgressDrawTarget::stdout();
    let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::stdout_with_hz(0u8));
    multi_progress.remove(&MultiProgress::default().insert(1usize, ProgressBar::with_draw_target(None, progress_draw_target)));
}
```

Run with `cargo run` in this directory (package `repro_indicatif_src_multi_rs_276_17_fa310c`).

## Output

```
thread 'main' (1268998) panicked at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/multi.rs:276:17:
assertion failed: Arc::ptr_eq(&self.state, state)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <indicatif::multi::MultiProgress>::remove
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/multi.rs:276:17
   4: repro_indicatif_src_multi_rs_276_17_fa310c::main
             at ./src/main.rs:8:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
