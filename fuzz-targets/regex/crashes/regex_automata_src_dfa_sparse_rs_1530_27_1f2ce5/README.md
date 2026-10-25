# regex-automata/src/dfa/sparse.rs:1530:27

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/sparse.rs:1530:27`
- last API call: `regex_automata::dfa::sparse::DFA<Vec<u8>>::next_state`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13340438766912005266

## Message

```
range end index 152 out of range for slice of length 138
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
use regex_automata::Anchored;
use regex_automata::dfa::dense::Builder;
use regex_automata::nfa::thompson::NFA;

fn main() {
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::next_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new("[[test]]\nname ").unwrap(), <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::universal_start_state(&Builder::default().build_from_nfa(&NFA::compiler().build("<svg xmtp:3.o182\"Bp-col\"/>iearG").unwrap()).unwrap(), Anchored::No).unwrap(), 0u8);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_sparse_rs_1530_27_1f2ce5`).

## Output

```
thread 'main' (367133) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1530:27:
range end index 152 out of range for slice of length 138
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
   5: <regex_automata::dfa::sparse::Transitions<alloc::vec::Vec<u8>>>::state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1530:27
   6: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>> as regex_automata::dfa::automaton::Automaton>::next_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1155:17
   7: repro_regex_automata_src_dfa_sparse_rs_1530_27_1f2ce5::main
             at ./src/main.rs:6:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
