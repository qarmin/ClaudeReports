# regex-automata/src/dfa/sparse.rs:2342:21

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/sparse.rs:2342:21`
- last API call: `regex_automata::dfa::sparse::DFA<Vec<u8>>::match_pattern`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12389961722238851207

## Message

```
attempt to multiply with overflow
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
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::never_match().unwrap(), StateID::from(0u8), 10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_sparse_rs_2342_21_44c474`).

## Output

```
thread 'main' (364688) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:2342:21:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <regex_automata::dfa::sparse::State>::pattern_id
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:2342:21
   4: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>> as regex_automata::dfa::automaton::Automaton>::match_pattern
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1192:27
   5: repro_regex_automata_src_dfa_sparse_rs_2342_21_44c474::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
