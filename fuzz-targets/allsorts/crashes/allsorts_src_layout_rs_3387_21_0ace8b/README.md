# allsorts/src/layout.rs:3387:21

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/layout.rs:3387:21`
- last API call: `allsorts::layout::Coverage::glyph_count`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 115756721065991852

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::layout::Coverage;

fn main() {
    let coverage = Coverage::Format2 { coverage_range_array: vec![<allsorts::layout::CoverageRangeRecord as allsorts::binary::read::ReadFrom>::read_from((1u16, 0u16, 0u16))] };
    let _ = coverage.glyph_count();
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_layout_rs_3387_21_0ace8b`).

## Output

```
thread 'main' (19509) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:3387:21:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::layout::Coverage>::glyph_count::{closure#0}
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:3387:21
   4: <core::slice::iter::Iter<allsorts::layout::CoverageRangeRecord> as core::iter::traits::iterator::Iterator>::fold::<usize, <allsorts::layout::Coverage>::glyph_count::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   5: <allsorts::layout::Coverage>::glyph_count
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/layout.rs:3386:18
   6: repro_allsorts_src_layout_rs_3387_21_0ace8b::main
             at ./src/main.rs:5:22
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
