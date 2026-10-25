# ron/src/util/span_substring.rs:78:58

- crate: https://github.com/ron-rs/ron @ 7b32209108a39f76192df7ef15701b3a48cd7a5f
- kind: panic
- location: `ron/src/util/span_substring.rs:78:58`
- last API call: `ron::de::Span::substring_inclusive`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
slice index starts at 9 but ends at 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ron = { git = "https://github.com/ron-rs/ron", rev = "7b32209108a39f76192df7ef15701b3a48cd7a5f", features = ["indexmap", "integer128", "internal-span-substring-test", "std", "unicode-segmentation"] }
```

`src/main.rs`:

```rust
use ron::de::Position;
use ron::de::Span;

fn main() {
    let span = Span { start: Position { line: 1usize, col: 10usize }, end: Position { line: 1usize, col: 1usize } };
    let _ = span.substring_inclusive("a5a0aaa7a");
}
```

Run with `cargo run` in this directory (package `repro_ron_src_util_span_substring_rs_78_58_dfd178`).

## Output

```
thread 'main' (548604) panicked at /home/rafal/.cargo/git/checkouts/ron-89c3157f0839f67d/7b32209/src/util/span_substring.rs:78:58:
slice index starts at 9 but ends at 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeInclusive<usize> as core::slice::index::SliceIndex<[&str]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:689:9
   4: <[&str] as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<&str> as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <ron::error::Span>::substring_inclusive
             at /home/rafal/.cargo/git/checkouts/ron-89c3157f0839f67d/7b32209/src/util/span_substring.rs:78:58
   7: repro_ron_src_util_span_substring_rs_78_58_dfd178::main
             at ./src/main.rs:6:18
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
