# regex-automata/src/dfa/sparse.rs:1514:39

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/sparse.rs:1514:39`
- last API call: `regex_automata::dfa::sparse::DFA<Vec<u8>>::next_eoi_state`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 256 out of range for slice of length 65
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
use regex_automata::util::primitives::StateID;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::next_eoi_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new("").unwrap(), StateID::from_ne_bytes([0u8, 1u8, 0u8, 0u8]).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_sparse_rs_1514_39_99e146`).

## Output

```
thread 'main' (461377) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1514:39:
range start index 256 out of range for slice of length 65
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[u8] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <regex_automata::dfa::sparse::Transitions<alloc::vec::Vec<u8>>>::state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1514:39
   6: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>> as regex_automata::dfa::automaton::Automaton>::next_eoi_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1169:17
   7: repro_regex_automata_src_dfa_sparse_rs_1514_39_99e146::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
