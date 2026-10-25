# regex-automata/src/dfa/dense.rs:3198:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/dense.rs:3198:9`
- last API call: `regex_automata::dfa::dense::DFA<Vec<u32>>::next_state`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14323609362838452677

## Message

```
index out of bounds: the len is 6 but the index is 8
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

fn main() {
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::next_state(&Builder::new().build_many::<String>(&[]).unwrap(), <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::universal_start_state(&Builder::new().build_many::<String>(&["".to_string()]).unwrap(), Anchored::No).unwrap(), 0u8);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_dense_rs_3198_9_311486`).

## Output

```
thread 'main' (332598) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3198:9:
index out of bounds: the len is 6 but the index is 8
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>> as regex_automata::dfa::automaton::Automaton>::next_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3198:9
   4: repro_regex_automata_src_dfa_dense_rs_3198_9_311486::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
