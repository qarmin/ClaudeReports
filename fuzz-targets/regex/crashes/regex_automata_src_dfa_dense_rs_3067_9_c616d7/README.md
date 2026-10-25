# regex-automata/src/dfa/dense.rs:3067:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/dfa/dense.rs:3067:9`
- last API call: `regex_automata::dfa::dense::DFA<Vec<u32>>::match_pattern`
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
use regex_automata::dfa::dense::Builder;
use regex_automata::nfa::thompson::backtrack::Config;

fn main() {
    let config = Config::new();
    let usize_v = config.get_visited_capacity();
    let _ = <regex_automata::dfa::dense::DFA<Vec<u32>> as regex_automata::dfa::Automaton>::match_pattern(&Builder::new().build_many::<String>(&["".to_string(), "".to_string()]).unwrap(), regex_automata::nfa::thompson::Builder::new().add_empty().unwrap(), usize_v);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_dfa_dense_rs_3067_9_c616d7`).

## Output

```
thread 'main' (461635) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3067:9:
assertion failed: self.is_match_state(id)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>>>::match_state_index
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3067:9
   4: <regex_automata::dfa::dense::DFA<alloc::vec::Vec<u32>> as regex_automata::dfa::automaton::Automaton>::match_pattern
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/dense.rs:3244:32
   5: repro_regex_automata_src_dfa_dense_rs_3067_9_c616d7::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
