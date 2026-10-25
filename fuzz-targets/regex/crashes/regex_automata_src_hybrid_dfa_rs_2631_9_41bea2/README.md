# regex-automata/src/hybrid/dfa.rs:2631:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/hybrid/dfa.rs:2631:9`
- last API call: `regex_automata::hybrid::dfa::DFA::start_state_reverse`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3547980371825937179

## Message

```
assertion failed: self.as_ref().is_valid(id)
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
use regex_automata::Input;
use regex_automata::hybrid::dfa::Builder;
use regex_automata::hybrid::dfa::Cache;
use regex_automata::meta::Regex;

fn main() {
    let regex = Regex::new_many::<String>(&[]).unwrap();
    let captures = regex.create_captures();
    let string = captures.interpolate_string("", "a");
    let builder = Builder::new();
    let d_f_a = builder.build(&string).unwrap();
    let mut cache = Cache::new(&Builder::new().build_many::<String>(&["".to_string()]).unwrap());
    let vec = vec![];
    let find_matches = regex.find_iter::<regex_automata::Input>(Input::new::<Vec<u8>>(&vec));
    let input = find_matches.input();
    let _ = d_f_a.start_state_reverse(&mut cache, input);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_hybrid_dfa_rs_2631_9_41bea2`).

## Output

```
thread 'main' (334885) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/dfa.rs:2631:9:
assertion failed: self.as_ref().is_valid(id)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <regex_automata::hybrid::dfa::Lazy>::set_start_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/dfa.rs:2631:9
   4: <regex_automata::hybrid::dfa::Lazy>::cache_start_group
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/dfa.rs:2196:14
   5: <regex_automata::hybrid::dfa::DFA>::start_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/dfa.rs:1574:32
   6: <regex_automata::hybrid::dfa::DFA>::start_state_reverse
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/dfa.rs:1636:14
   7: repro_regex_automata_src_hybrid_dfa_rs_2631_9_41bea2::main
             at ./src/main.rs:16:19
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
