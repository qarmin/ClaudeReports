# text-size/src/range.rs:69:46

- crate: https://github.com/rust-analyzer/text-size @ f5048daacdf8a8b7b6ff61572f75b42b8fe657ba
- kind: panic
- location: `text-size/src/range.rs:69:46`
- last API call: `text_size::TextRange::at`
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
text-size = { git = "https://github.com/rust-analyzer/text-size", rev = "f5048daacdf8a8b7b6ff61572f75b42b8fe657ba" }
```

`src/main.rs`:

```rust
use text_size::TextRange;
use text_size::TextSize;

fn main() {
    let _ = TextRange::at(TextSize::new(u32::MAX), TextSize::try_from(1usize).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_text_size_src_range_rs_69_46_ad59d0`).

## Output

```
thread 'main' (528812) panicked at /home/rafal/.cargo/git/checkouts/text-size-916c6c9c706349c1/f5048da/src/range.rs:69:46:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <text_size::range::TextRange>::at
             at /home/rafal/.cargo/git/checkouts/text-size-916c6c9c706349c1/f5048da/src/range.rs:69:46
   4: repro_text_size_src_range_rs_69_46_ad59d0::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
