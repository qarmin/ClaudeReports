# regex-automata/src/dfa/dense.rs:3016:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/dense.rs:3016:9`
- last API call: `regex_automata::dfa::dense::DFA<Vec<u32>>::match_len`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.is_match_state(id)
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
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::match_len(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::new_many::<String>(&[]).unwrap(), StateID::from(0u8));
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_dense_rs_3016_9_0b533d`).

## Output

```
thread 'main' (461585) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3016:9:
assertion failed: self.is_match_state(id)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>>>::match_pattern_len
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3016:9
   4: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>> as regex_automata::dfa::automaton::Automaton>::match_len
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3231:14
   5: repro_regex_automata_src_dfa_dense_rs_3016_9_0b533d::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
