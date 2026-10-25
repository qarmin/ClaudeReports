# regex-automata/src/util/prefilter/memmem.rs:47:39

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/util/prefilter/memmem.rs:47:39`
- last API call: `regex_automata::util::prefilter::Prefilter::find`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15769457413697744281

## Message

```
range end index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-syntax = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-automata = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
```

`src/main.rs`:

```rust
use regex_automata::MatchKind;
use regex_automata::Span;
use regex_automata::util::prefilter::Prefilter;
use regex_syntax::Parser;

fn main() {
    let prefilter = Prefilter::from_hir_prefix(MatchKind::All, &Parser::new().parse("sv").unwrap()).unwrap();
    let _ = prefilter.find(&vec![], Span { start: 0usize, end: 1usize });
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_util_prefilter_memmem_rs_47_39_6c1e48`).

## Output

```
thread 'main' (331520) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/prefilter/memmem.rs:47:39:
range end index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[u8] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <[u8] as core::ops::index::Index<regex_automata::util::search::Span>>::index
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/search.rs:863:14
   6: <regex_automata::util::prefilter::memmem::Memmem as regex_automata::util::prefilter::PrefilterI>::find
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/prefilter/memmem.rs:47:39
   7: <alloc::sync::Arc<dyn regex_automata::util::prefilter::PrefilterI> as regex_automata::util::prefilter::PrefilterI>::find
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/prefilter/mod.rs:511:18
   8: <regex_automata::util::prefilter::Prefilter>::find
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/prefilter/mod.rs:356:22
   9: repro_regex_automata_src_util_prefilter_memmem_rs_47_39_6c1e48::main
             at ./src/main.rs:8:23
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
