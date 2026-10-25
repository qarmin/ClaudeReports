# regex-automata/src/meta/wrappers.rs:653:49

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/meta/wrappers.rs:653:49`
- last API call: `regex_automata::meta::Regex::search_half_with`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6046615434485089934

## Message

```
called `Option::unwrap()` on a `None` value
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
use regex_automata::meta::Cache;
use regex_automata::meta::Regex;

fn main() {
    let regex = Regex::new("; hm4K]l&\u{3fec0}*f\u{934b4} ").unwrap();
    let mut cache = Cache::new(&Regex::builder().build("").unwrap());
    let vec = vec![108u8, 66u8, 0u8, 124u8, 0u8, 0u8, 0u8, 0u8, 0u8, 107u8, 0u8, 56u8, 47u8, 0u8, 0u8];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = regex.search_half_with(&mut cache, &input);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_meta_wrappers_rs_653_49_1511e9`).

## Output

```
thread 'main' (368127) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/wrappers.rs:653:49:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&mut regex_automata::hybrid::regex::Cache>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <regex_automata::meta::wrappers::HybridEngine>::try_search_half_fwd
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/wrappers.rs:653:49
   6: <regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy>::search_half
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/strategy.rs:759:21
   7: <regex_automata::meta::regex::Regex>::search_half_with
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/regex.rs:1301:24
   8: repro_regex_automata_src_meta_wrappers_rs_653_49_1511e9::main
             at ./src/main.rs:10:19
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
