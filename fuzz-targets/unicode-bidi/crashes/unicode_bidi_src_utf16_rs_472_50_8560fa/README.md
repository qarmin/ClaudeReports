# unicode-bidi/src/utf16.rs:472:50

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/utf16.rs:472:50`
- last API call: `unicode_bidi::utf16::ParagraphBidiInfo::reordered_levels_per_char`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
slice index starts at 1 but ends at 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
unicode-bidi = { git = "https://github.com/servo/unicode-bidi", rev = "9a8c7b03be01def5cf7014b89d5af90f7382fdad" }
```

`src/main.rs`:

```rust
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::ParagraphBidiInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: 65535usize..65535usize, level: Level::new_explicit(0u8).unwrap() };
    let usize_v = paragraph_info.len();
    let vec = vec![0u16];
    let paragraph_bidi_info = ParagraphBidiInfo::new(&vec, None);
    let _ = paragraph_bidi_info.reordered_levels_per_char(1usize..usize_v);
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_utf16_rs_472_50_8560fa`).

## Output

```
thread 'main' (1245964) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/utf16.rs:472:50:
slice index starts at 1 but ends at 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[unicode_bidi::char_data::tables::BidiClass]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[unicode_bidi::char_data::tables::BidiClass] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<unicode_bidi::char_data::tables::BidiClass> as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <unicode_bidi::utf16::ParagraphBidiInfo>::reordered_levels
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/utf16.rs:472:50
   7: <unicode_bidi::utf16::ParagraphBidiInfo>::reordered_levels_per_char
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/utf16.rs:494:27
   8: repro_unicode_bidi_src_utf16_rs_472_50_8560fa::main
             at ./src/main.rs:10:33
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
