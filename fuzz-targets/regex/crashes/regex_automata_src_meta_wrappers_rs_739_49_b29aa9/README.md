# regex-automata/src/meta/wrappers.rs:739:49

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/meta/wrappers.rs:739:49`
- last API call: `regex_automata::meta::Regex::which_overlapping_matches_with`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15011784665438834213

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
use regex_automata::PatternSet;
use regex_automata::meta::Builder;
use regex_automata::meta::Regex;
use regex_automata::util::captures::GroupInfo;

fn main() {
    let group_info = GroupInfo::default();
    let usize_v = group_info.explicit_slot_len();
    let regex = Regex::new_many::<String>(&["\u{ff0c6}\u{5c4ee}\u{200b}|R.".to_string()]).unwrap();
    let builder = Builder::new();
    let regex2 = builder.build_many::<String>(&[]).unwrap();
    let mut cache = regex2.create_cache();
    let mut pattern_set = PatternSet::new(usize_v);
    let vec = vec![73u8, 70u8];
    let input = Input::new::<Vec<u8>>(&vec);
    regex.which_overlapping_matches_with(&mut cache, &input, &mut pattern_set);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_meta_wrappers_rs_739_49_b29aa9`).

## Output

```
thread 'main' (380647) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/wrappers.rs:739:49:
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
   5: <regex_automata::meta::wrappers::HybridEngine>::try_which_overlapping_matches
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/wrappers.rs:739:49
   6: <regex_automata::meta::strategy::Core as regex_automata::meta::strategy::Strategy>::which_overlapping_matches
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/strategy.rs:889:32
   7: <regex_automata::meta::regex::Regex>::which_overlapping_matches_with
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/meta/regex.rs:1505:24
   8: repro_regex_automata_src_meta_wrappers_rs_739_49_b29aa9::main
             at ./src/main.rs:17:11
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
