# regex-automata/src/nfa/thompson/pikevm.rs:2157:17

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/nfa/thompson/pikevm.rs:2157:17`
- last API call: `regex_automata::nfa::thompson::pikevm::PikeVM::search`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16556884273959434312

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-syntax = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
regex-automata = { git = "https://github.com/rust-lang/regex", rev = "72d650cb0a880a01ab6dc2137c0888e8f89740f7" }
```

`src/main.rs`:

```rust
use regex_automata::Input;
use regex_automata::meta::Builder;
use regex_automata::nfa::thompson::pikevm::PikeVM;
use regex_syntax::hir::Dot;
use regex_syntax::hir::Hir;

fn main() {
    let pike_v_m = PikeVM::never_match().unwrap();
    let mut cache = pike_v_m.create_cache();
    let builder = Builder::new();
    let regex = builder.build_from_hir(&Hir::dot(Dot::AnyCharExcept('a'))).unwrap();
    let vec = vec![];
    let captures_matches = regex.captures_iter::<regex_automata::Input>(Input::new::<Vec<u8>>(&vec));
    let input = captures_matches.input();
    let builder2 = regex_automata::dfa::onepass::Builder::new();
    let d_f_a = builder2.build("").unwrap();
    let mut captures = d_f_a.create_captures();
    pike_v_m.search(&mut cache, input, &mut captures);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_nfa_thompson_pikevm_rs_2157_17_85f53b`).

## Output

```
thread 'main' (338697) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:2157:17:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <regex_automata::nfa::thompson::pikevm::SlotTable>::all_absent
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:2157:17
   4: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1363:45
   5: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1146:29
   6: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1099:27
   7: <regex_automata::nfa::thompson::pikevm::PikeVM>::search
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1027:24
   8: repro_regex_automata_src_nfa_thompson_pikevm_rs_2157_17_85f53b::main
             at ./src/main.rs:18:14
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
