# percent_encoding/src/ascii_set.rs:64:9

- crate: https://github.com/servo/rust-url @ 00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a
- kind: panic
- location: `percent_encoding/src/ascii_set.rs:64:9`
- last API call: `percent_encoding::AsciiSet::remove`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 4 but the index is 7
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
url = { git = "https://github.com/servo/rust-url", rev = "00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a" }
percent-encoding = { git = "https://github.com/servo/rust-url", rev = "00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a" }
```

`src/main.rs`:

```rust
use percent_encoding::AsciiSet;

fn main() {
    let ascii_set = AsciiSet::EMPTY;
    let _ = ascii_set.remove(255u8);
}
```

Run with `cargo run` in this directory (package `repro_percent_encoding_src_ascii_set_rs_64_9_21430f`).

## Output

```
thread 'main' (1393432) panicked at /home/rafal/.cargo/git/checkouts/rust-url-f0546b39189470a0/00a6ce5/percent_encoding/src/ascii_set.rs:64:9:
index out of bounds: the len is 4 but the index is 7
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <percent_encoding::ascii_set::AsciiSet>::remove
             at /home/rafal/.cargo/git/checkouts/rust-url-f0546b39189470a0/00a6ce5/percent_encoding/src/ascii_set.rs:64:9
   4: repro_percent_encoding_src_ascii_set_rs_64_9_21430f::main
             at ./src/main.rs:5:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
