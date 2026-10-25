# regex-automata/src/dfa/sparse.rs:1521:35

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/sparse.rs:1521:35`
- last API call: `regex_automata::dfa::sparse::DFA<Vec<u8>>::match_pattern`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16111419052064828835

## Message

```
mid > len
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
    let _ = <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::match_pattern(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[include_str!("string_0.txt").to_string(), " ".to_string(), "\u{200b}\u{200d}".to_string()]).unwrap(), StateID::try_from(2049u32).unwrap(), 1usize);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_sparse_rs_1521_35_6ad61f`).

## Output

```
thread 'main' (378334) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1521:35:
mid > len
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[u8]>::split_at
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1964:21
   3: <regex_automata::dfa::sparse::Transitions<alloc::vec::Vec<u8>>>::state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1521:35
   4: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>> as regex_automata::dfa::automaton::Automaton>::match_pattern
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:1192:17
   5: repro_regex_automata_src_dfa_sparse_rs_1521_35_6ad61f::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
