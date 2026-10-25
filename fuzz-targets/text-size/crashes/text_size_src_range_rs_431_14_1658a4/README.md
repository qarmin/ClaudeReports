# text-size/src/range.rs:431:14

- crate: https://github.com/rust-analyzer/text-size @ f5048daacdf8a8b7b6ff61572f75b42b8fe657ba
- kind: panic
- location: `text-size/src/range.rs:431:14`
- last API call: `text_size::TextRange::sub<text_size::TextSize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
TextRange -offset overflowed
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
    let text_size = TextSize::from(0u32);
    let text_range = TextRange::empty(text_size);
    let text_range2 = text_range.cover_offset(text_size);
    let text_range3 = text_range2.cover(text_range);
    let _ = <text_size::TextRange as ::core::ops::Sub<text_size::TextSize>>::sub(text_range3, TextSize::try_from(1usize).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_text_size_src_range_rs_431_14_1658a4`).

## Output

```
thread 'main' (528900) panicked at /home/rafal/.cargo/git/checkouts/text-size-916c6c9c706349c1/f5048da/src/range.rs:431:14:
TextRange -offset overflowed
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<text_size::range::TextRange>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <text_size::range::TextRange as core::ops::arith::Sub<text_size::size::TextSize>>::sub
             at /home/rafal/.cargo/git/checkouts/text-size-916c6c9c706349c1/f5048da/src/range.rs:431:14
   6: repro_text_size_src_range_rs_431_14_1658a4::main
             at ./src/main.rs:9:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
