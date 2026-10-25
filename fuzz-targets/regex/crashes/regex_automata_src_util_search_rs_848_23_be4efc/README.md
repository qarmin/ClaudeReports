# regex-automata/src/util/search.rs:848:23

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/util/search.rs:848:23`
- last API call: `regex_automata::Span::offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-automata = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
```

`src/main.rs`:

```rust
use regex_automata::Span;

fn main() {
    let span = Span { start: 0usize, end: 0usize };
    let span2 = span.offset(1usize);
    let _ = span2.offset(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_util_search_rs_848_23_be4efc`).

## Output

```
thread 'main' (461540) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/search.rs:848:23:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <regex_automata::util::search::Span>::offset
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/search.rs:848:23
   4: repro_regex_automata_src_util_search_rs_848_23_be4efc::main
             at ./src/main.rs:6:19
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
