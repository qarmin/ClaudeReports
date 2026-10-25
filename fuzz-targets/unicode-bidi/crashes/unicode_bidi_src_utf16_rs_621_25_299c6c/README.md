# unicode-bidi/src/utf16.rs:621:25

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/utf16.rs:621:25`
- last API call: `unicode_bidi::utf16::Paragraph::level_at`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
unicode-bidi = { git = "https://github.com/servo/unicode-bidi", rev = "9a8c7b03be01def5cf7014b89d5af90f7382fdad" }
```

`src/main.rs`:

```rust
use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::BidiInfo;
use unicode_bidi::utf16::Paragraph;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let vec = vec![];
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &vec, None);
    let paragraph_info = ParagraphInfo { range: 0usize..0usize, level: Level::ltr() };
    let paragraph = Paragraph::new(&bidi_info, &paragraph_info);
    let _ = paragraph.level_at(0usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_utf16_rs_621_25_299c6c`).

## Output

```
thread 'main' (1244590) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/utf16.rs:621:25:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[unicode_bidi::level::Level]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[unicode_bidi::level::Level] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<unicode_bidi::level::Level> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <unicode_bidi::utf16::Paragraph>::level_at
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/utf16.rs:621:25
   7: repro_unicode_bidi_src_utf16_rs_621_25_299c6c::main
             at ./src/main.rs:13:23
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
