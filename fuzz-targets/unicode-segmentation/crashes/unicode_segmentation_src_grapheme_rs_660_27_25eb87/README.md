# unicode-segmentation/src/grapheme.rs:660:27

- crate: https://github.com/unicode-rs/unicode-segmentation @ e77d18649cbc109fefd15a9d7db8320480abfd4e
- kind: panic
- location: `unicode-segmentation/src/grapheme.rs:660:27`
- last API call: `unicode_segmentation::GraphemeCursor::is_boundary`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
start byte index 1 is not a char boundary; it is inside '\u{1c30b}' (bytes 0..4 of string)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
unicode-segmentation = { git = "https://github.com/unicode-rs/unicode-segmentation", rev = "e77d18649cbc109fefd15a9d7db8320480abfd4e" }
```

`src/main.rs`:

```rust
use unicode_segmentation::GraphemeCursor;

fn main() {
    let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, false);
    let _ = grapheme_cursor.is_boundary("\u{1c30b}", 0usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_segmentation_src_grapheme_rs_660_27_25eb87`).

## Output

```
thread 'main' (1288878) panicked at /home/rafal/.cargo/git/checkouts/unicode-segmentation-ebec98fda7263355/e77d186/src/grapheme.rs:660:27:
start byte index 1 is not a char boundary; it is inside '\u{1c30b}' (bytes 0..4 of string)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:528:21
   5: <str as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:63:15
   6: <unicode_segmentation::grapheme::GraphemeCursor>::is_boundary
             at /home/rafal/.cargo/git/checkouts/unicode-segmentation-ebec98fda7263355/e77d186/src/grapheme.rs:660:27
   7: repro_unicode_segmentation_src_grapheme_rs_660_27_25eb87::main
             at ./src/main.rs:5:29
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
