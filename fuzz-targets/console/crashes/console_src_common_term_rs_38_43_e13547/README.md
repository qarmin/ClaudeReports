# console/src/common_term.rs:38:43

- crate: https://github.com/console-rs/console @ 4329b774d2e7117df706193636b4734b147e6554
- kind: panic
- location: `console/src/common_term.rs:38:43`
- last API call: `console::Term::move_cursor_to`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
console = { git = "https://github.com/console-rs/console", rev = "4329b774d2e7117df706193636b4734b147e6554" }
```

`src/main.rs`:

```rust
use console::Term;

fn main() {
    let term = Term::buffered_stderr();
    let _ = term.move_cursor_to(0usize, usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_console_src_common_term_rs_38_43_e13547`).

## Output

```
thread 'main' (522785) panicked at /home/rafal/.cargo/git/checkouts/console-0669b44a1ef26b6b/4329b77/src/common_term.rs:38:43:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: console::common_term::move_cursor_to
             at /home/rafal/.cargo/git/checkouts/console-0669b44a1ef26b6b/4329b77/src/common_term.rs:38:43
   4: <console::term::Term>::move_cursor_to
             at /home/rafal/.cargo/git/checkouts/console-0669b44a1ef26b6b/4329b77/src/term.rs:431:9
   5: repro_console_src_common_term_rs_38_43_e13547::main
             at ./src/main.rs:5:18
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
