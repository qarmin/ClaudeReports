# timeout in regex_automata::dfa::regex::Regex<regex_automata::dfa::sparse::DFA<Vec<u8>>>::reverse

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: timeout
- last API call: `regex_automata::dfa::regex::Regex<regex_automata::dfa::sparse::DFA<Vec<u8>>>::reverse`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15168920510655917674

## Message

```
process ended abnormally (timeout) while calling regex_automata::dfa::regex::Regex<regex_automata::dfa::sparse::DFA<Vec<u8>>>::reverse
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
use regex_automata::dfa::regex::Builder;

fn main() {
    let regex = Builder::new().build_many_sparse::<String>(&[include_str!("string_0.txt").to_string(), "\u{200b}\u{200d}".to_string(), include_str!("string_1.txt").to_string(), include_str!("string_2.txt").to_string(), "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"166\" viewBox=\"0 0 16 37\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><\"0.5\"/></linearGradient><".to_string(), include_str!("string_3.txt").to_string()]).unwrap();
    let _ = regex.reverse();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_regex_automata_dfa_regex_regex_regex_automata_dfa_8f16b6`).

