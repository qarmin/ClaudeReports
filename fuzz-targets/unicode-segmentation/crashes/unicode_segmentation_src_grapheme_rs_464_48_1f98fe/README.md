# unicode-segmentation/src/grapheme.rs:464:48

- crate: https://github.com/unicode-rs/unicode-segmentation @ e77d18649cbc109fefd15a9d7db8320480abfd4e
- kind: panic
- location: `unicode-segmentation/src/grapheme.rs:464:48`
- last API call: `unicode_segmentation::GraphemeCursor::provide_context`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
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
    let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, true);
    let usize_v = grapheme_cursor.cur_cursor();
    let _ = grapheme_cursor.is_boundary("a", usize_v);
    grapheme_cursor.provide_context("", 1usize);
}
```

Run with `cargo run` in this directory (package `repro_unicode_segmentation_src_grapheme_rs_464_48_1f98fe`).

## Output

```
thread 'main' (1288992) panicked at /home/rafal/.cargo/git/checkouts/unicode-segmentation-ebec98fda7263355/e77d186/src/grapheme.rs:464:48:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<char>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <unicode_segmentation::grapheme::GraphemeCursor>::provide_context
             at /home/rafal/.cargo/git/checkouts/unicode-segmentation-ebec98fda7263355/e77d186/src/grapheme.rs:464:48
   6: repro_unicode_segmentation_src_grapheme_rs_464_48_1f98fe::main
             at ./src/main.rs:7:21
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
