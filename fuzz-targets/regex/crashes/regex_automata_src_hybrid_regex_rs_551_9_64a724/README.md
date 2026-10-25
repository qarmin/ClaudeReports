# regex-automata/src/hybrid/regex.rs:551:9

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/hybrid/regex.rs:551:9`
- last API call: `regex_automata::hybrid::regex::Regex::pattern_len`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4403675013845830486

## Message

```
assertion `left == right` failed
  left: 0
 right: 1
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
use regex_automata::hybrid::dfa::DFA;
use regex_automata::hybrid::regex::Builder;

fn main() {
    let builder = Builder::default();
    let regex = builder.build_from_dfas(DFA::never_match().unwrap(), DFA::new("").unwrap());
    let _ = regex.pattern_len();
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_hybrid_regex_rs_551_9_64a724`).

## Output

```
thread 'main' (338063) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/regex.rs:551:9:
assertion `left == right` failed
  left: 0
 right: 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <regex_automata::hybrid::regex::Regex>::pattern_len
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/hybrid/regex.rs:551:9
   5: repro_regex_automata_src_hybrid_regex_rs_551_9_64a724::main
             at ./src/main.rs:7:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
