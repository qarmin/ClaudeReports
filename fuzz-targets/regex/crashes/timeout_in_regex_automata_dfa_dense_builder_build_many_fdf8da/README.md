# timeout in regex_automata::dfa::dense::Builder::build_many

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: timeout
- last API call: `regex_automata::dfa::dense::Builder::build_many`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10376729314649029274

## Message

```
process ended abnormally (timeout) while calling regex_automata::dfa::dense::Builder::build_many
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
use regex_automata::nfa::thompson::NFA;
use regex_automata::util::primitives::SmallIndex;

fn main() {
    let builder = Builder::new();
    let _ = builder.build_many::<String>(&["=9B^ve\u{6874c}Y4\u{cbc25}'?\u{41f4f}`;'😀\u{df101}\u{a992b}dZ\u{fbdbf}".to_string(), "\u{8e4b2}]?6%\u{2ff58}JL\u{f670b}*rE87zH-\u{109d6f}- \u{353a1}\u{1a14f}\u{e96c3}".to_string(), "a\u{301}\u{301}\u{301}".to_string(), include_str!("string_0.txt").to_string()]);
    let _ = SmallIndex::try_from(1000u32);
    let compiler = NFA::compiler();
    let n_f_a = compiler.build_many::<String>(&["Zażółć gęślą jaźń".to_string()]).unwrap();
    let byte_classes = n_f_a.byte_classes();
    let _ = byte_classes.iter();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_regex_automata_dfa_dense_builder_build_many_fdf8da`).

