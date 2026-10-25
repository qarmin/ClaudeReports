# sec1/src/point.rs:556:9

- crate: https://github.com/RustCrypto/formats @ 13a0390d40297121e272044ee73d796e1bfb72ee
- kind: panic
- location: `sec1/src/point.rs:556:9`
- last API call: `sec1::point::Tag::message_len`
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
sec1 = { git = "https://github.com/RustCrypto/formats", rev = "13a0390d40297121e272044ee73d796e1bfb72ee" }
```

`src/main.rs`:

```rust
use sec1::point::Tag;

fn main() {
    let tag = Tag::Compact;
    let _ = tag.message_len(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_sec1_src_point_rs_556_9_7f1ae6`).

## Output

```
thread 'main' (1363051) panicked at /home/rafal/.cargo/git/checkouts/formats-0f4cf7e5de45495c/13a0390/sec1/src/point.rs:556:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <sec1::point::Tag>::message_len
             at /home/rafal/.cargo/git/checkouts/formats-0f4cf7e5de45495c/13a0390/sec1/src/point.rs:556:9
   4: repro_sec1_src_point_rs_556_9_7f1ae6::main
             at ./src/main.rs:5:17
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
