# indicatif/src/style.rs:877:30

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: panic
- location: `indicatif/src/style.rs:877:30`
- last API call: `indicatif::ProgressStyle::progress_chars`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed: got passed un-equal width progress characters
  left: 2
 right: 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
indicatif = { git = "https://github.com/console-rs/indicatif", rev = "6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9" }
```

`src/main.rs`:

```rust
use indicatif::ProgressStyle;

fn main() {
    let progress_style = ProgressStyle::default_bar();
    let progress_style2 = progress_style.clone().template("").unwrap();
    let _ = progress_style2.clone().progress_chars("\u{3ffb6}\"");
}
```

Run with `cargo run` in this directory (package `repro_indicatif_src_style_rs_877_30_f03593`).

## Output

```
thread 'main' (1269169) panicked at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:877:30:
assertion `left == right` failed: got passed un-equal width progress characters
  left: 2
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: indicatif::style::width::{closure#1}
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:877:30
   5: core::iter::adapters::map::map_fold::<&alloc::boxed::Box<str>, usize, core::option::Option<usize>, indicatif::style::width::{closure#0}, indicatif::style::width::{closure#1}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:21
   6: <core::slice::iter::Iter<alloc::boxed::Box<str>> as core::iter::traits::iterator::Iterator>::fold::<core::option::Option<usize>, core::iter::adapters::map::map_fold<&alloc::boxed::Box<str>, usize, core::option::Option<usize>, indicatif::style::width::{closure#0}, indicatif::style::width::{clos... (+21 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   7: <core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::boxed::Box<str>>, indicatif::style::width::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<core::option::Option<usize>, indicatif::style::width::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   8: indicatif::style::width
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:874:10
   9: <indicatif::style::ProgressStyle>::progress_chars
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/style.rs:140:27
  10: repro_indicatif_src_style_rs_877_30_f03593::main
             at ./src/main.rs:6:37
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
