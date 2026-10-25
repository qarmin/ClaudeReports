# regex-automata/src/dfa/dense.rs:3221:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/dense.rs:3221:9`
- last API call: `regex_automata::dfa::dense::DFA<Vec<u32>>::next_eoi_state`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16336073676008258539

## Message

```
index out of bounds: the len is 6 but the index is 19
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
use regex_automata::util::start::Config;

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::next_eoi_state(&<regex_automata::dfa::dense::DFA<Vec<u32>>>::never_match().unwrap(), <regex_automata::dfa::sparse::DFA<Vec<u8>> as regex_automata::dfa::Automaton>::start_state(&<regex_automata::dfa::sparse::DFA<Vec<u8>>>::new_many::<String>(&[]).unwrap(), &Config::new()).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_dense_rs_3221_9_f5b9a5`).

## Output

```
thread 'main' (332135) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3221:9:
index out of bounds: the len is 6 but the index is 19
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>> as regex_automata::dfa::automaton::Automaton>::next_eoi_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3221:9
   4: repro_regex_automata_src_dfa_dense_rs_3221_9_f5b9a5::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
