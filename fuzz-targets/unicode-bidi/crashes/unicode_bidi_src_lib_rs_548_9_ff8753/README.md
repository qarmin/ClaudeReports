# unicode-bidi/src/lib.rs:548:9

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/lib.rs:548:9`
- last API call: `unicode_bidi::BidiInfo::visual_runs`
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
use unicode_bidi::BidiInfo;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let string = "".to_string();
    let bidi_info = BidiInfo::new(&string, None);
    let _ = bidi_info.visual_runs(&ParagraphInfo { range: 0usize..0usize, level: Level::from(0u8) }, 0usize..1usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_lib_rs_548_9_ff8753`).

## Output

```
thread 'main' (339939) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:548:9:
assertion failed: line.end <= self.levels.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <unicode_bidi::BidiInfo>::reordered_levels
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:548:9
   4: <unicode_bidi::BidiInfo>::visual_runs
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:674:27
   5: repro_unicode_bidi_src_lib_rs_548_9_ff8753::main
             at ./src/main.rs:8:23
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
