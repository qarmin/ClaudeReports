# vte/src/ansi.rs:460:29

- crate: https://github.com/alacritty/vte @ abeae765dd546dfff60b278f0757dcc71beb8ab1
- kind: panic
- location: `vte/src/ansi.rs:460:29`
- last API call: `vte::ansi::StdSyncHandler::set_timeout`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
overflow when adding duration to instant
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vte = { git = "https://github.com/alacritty/vte", rev = "abeae765dd546dfff60b278f0757dcc71beb8ab1", features = ["ansi", "bitflags", "cursor-icon", "log", "serde", "std"] }
```

`src/main.rs`:

```rust
use vte::ansi::StdSyncHandler;

fn main() {
    let mut std_sync_handler = StdSyncHandler::default();
    <vte::ansi::StdSyncHandler as vte::ansi::Timeout>::set_timeout(&mut std_sync_handler, std::time::Duration::new(10000000000000000000u64, 33u32 % 1_000_000_000));
}
```

Run with `cargo run` in this directory (package `repro_vte_src_ansi_rs_460_29_3534db`).

## Output

```
thread 'main' (568435) panicked at /home/rafal/.cargo/git/checkouts/vte-8bfa15be98f535f7/abeae76/src/ansi.rs:460:29:
overflow when adding duration to instant
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<std::time::Instant>>::expect
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:969:21
   5: <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/time.rs:428:33
   6: <vte::ansi::StdSyncHandler as vte::ansi::Timeout>::set_timeout
             at /home/rafal/.cargo/git/checkouts/vte-8bfa15be98f535f7/abeae76/src/ansi.rs:460:29
   7: repro_vte_src_ansi_rs_460_29_3534db::main
             at ./src/main.rs:5:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
