# unicode-bidi/src/lib.rs:829:29

- crate: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad
- kind: panic
- location: `unicode-bidi/src/lib.rs:829:29`
- last API call: `unicode_bidi::ParagraphBidiInfo::reorder_line`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
end byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
unicode-bidi = { git = "https://github.com/servo/unicode-bidi", rev = "9a8c7b03be01def5cf7014b89d5af90f7382fdad" }
```

`src/main.rs`:

```rust
use unicode_bidi::ParagraphBidiInfo;

fn main() {
    let string = "\u{feff}".to_string();
    let paragraph_bidi_info = ParagraphBidiInfo::new(&string, None);
    let _ = paragraph_bidi_info.reorder_line(0usize..1usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_bidi_src_lib_rs_829_29_863742`).

## Output

```
thread 'main' (1245194) panicked at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:829:29:
end byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <str as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   7: <unicode_bidi::ParagraphBidiInfo>::reorder_line
             at /home/rafal/.cargo/git/checkouts/unicode-bidi-19cde379c57bfc50/9a8c7b0/src/lib.rs:829:29
   8: repro_unicode_bidi_src_lib_rs_829_29_863742::main
             at ./src/main.rs:6:33
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
