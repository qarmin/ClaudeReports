# regex-automata/src/nfa/thompson/pikevm.rs:2149:24

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/nfa/thompson/pikevm.rs:2149:24`
- last API call: `regex_automata::nfa::thompson::pikevm::PikeVM::search_slots`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 829606305105343914

## Message

```
range end index 9 out of range for slice of length 8
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
use regex_automata::nfa::thompson::pikevm::PikeVM;

fn main() {
    let pike_v_m = PikeVM::always_match().unwrap();
    let mut cache = pike_v_m.create_cache();
    let vec = vec![];
    let input = Input::new::<Vec<u8>>(&vec);
    let _ = pike_v_m.search_slots(&mut cache, &input, &mut vec![None, None, None, None, None]);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_nfa_thompson_pikevm_rs_2149_24_d8a348`).

## Output

```
thread 'main' (360467) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:2149:24:
range end index 9 out of range for slice of length 8
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[core::option::Option<regex_automata::util::primitives::NonMaxUsize>]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:427:13
   4: <[core::option::Option<regex_automata::util::primitives::NonMaxUsize>] as core::ops::index::IndexMut<core::ops::range::Range<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<core::option::Option<regex_automata::util::primitives::NonMaxUsize>> as core::ops::index::IndexMut<core::ops::range::Range<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <regex_automata::nfa::thompson::pikevm::SlotTable>::for_state
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:2149:24
   7: <regex_automata::nfa::thompson::pikevm::PikeVM>::epsilon_closure_explore
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1692:37
   8: <regex_automata::nfa::thompson::pikevm::PikeVM>::epsilon_closure
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1631:26
   9: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1364:22
  10: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1146:29
  11: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1099:27
  12: repro_regex_automata_src_nfa_thompson_pikevm_rs_2149_24_d8a348::main
             at ./src/main.rs:9:22
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
