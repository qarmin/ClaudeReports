# unicode-bidi/src/lib.rs:792:9

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/lib.rs:792:9`
- last API call: `unicode_bidi::ParagraphBidiInfo::reordered_levels`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: line.end <= self.levels.len()
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
use unicode_bidi::ParagraphBidiInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let string = "a".to_string();
    let paragraph_bidi_info = ParagraphBidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &string, None);
    let _ = paragraph_bidi_info.reordered_levels(1usize..10usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_lib_rs_792_9_8c9fca`).

## Output

```
thread 'main' (1245094) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:792:9:
assertion failed: line.end <= self.levels.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <unicode_bidi::ParagraphBidiInfo>::reordered_levels
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:792:9
   4: repro_unicode_bidi_src_lib_rs_792_9_8c9fca::main
             at ./src/main.rs:8:33
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
