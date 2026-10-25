# mp4parse/src/lib.rs:415:17

- crate: https://github.com/mozilla/mp4parse-rust @ 95d98b9a6840f5bfc9a2323b3fa3f56d7f3c6077
- kind: panic
- location: `mp4parse/src/lib.rs:415:17`
- last API call: `mp4parse::Error::from<mp4parse::Status>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Status -> Error is only for Status:InvalidData errors
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
mp4parse = { git = "https://github.com/mozilla/mp4parse-rust", rev = "95d98b9a6840f5bfc9a2323b3fa3f56d7f3c6077" }
```

`src/main.rs`:

```rust
use mp4parse::Error;
use mp4parse::Status;

fn main() {
    let _ = Error::from(Status::Oom);
}
```

Run with `cargo run` in this directory (package `repro_mp4parse_src_lib_rs_415_17_3dc9b3`).

## Output

```
thread 'main' (1409985) panicked at /home/rafal/.cargo/git/checkouts/mp4parse-rust-aef5116aed75b335/95d98b9/mp4parse/src/lib.rs:415:17:
Status -> Error is only for Status:InvalidData errors
stack backtrace:
   0: std::panicking::begin_panic::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:751:5
   1: <mp4parse::Error as core::convert::From<mp4parse::Status>>::from
             at /home/rafal/.cargo/git/checkouts/mp4parse-rust-aef5116aed75b335/95d98b9/mp4parse/src/lib.rs:415:17
   2: repro_mp4parse_src_lib_rs_415_17_3dc9b3::main
             at ./src/main.rs:5:13
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
