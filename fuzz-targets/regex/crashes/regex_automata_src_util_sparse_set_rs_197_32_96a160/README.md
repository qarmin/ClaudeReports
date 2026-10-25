# regex-automata/src/util/sparse_set.rs:197:32

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/util/sparse_set.rs:197:32`
- last API call: `regex_automata::nfa::thompson::pikevm::PikeVM::captures`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 5 but the index is 8
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
use regex_automata::nfa::thompson::pikevm::Cache;
use regex_automata::nfa::thompson::pikevm::PikeVM;
use regex_automata::util::captures::Captures;
use regex_automata::util::captures::GroupInfo;

fn main() {
    let builder = PikeVM::builder();
    let pike_v_m = builder.build_many::<String>(&["".to_string(), "".to_string()]).unwrap();
    let mut cache = Cache::new(&PikeVM::new("").unwrap());
    let mut captures = Captures::all(GroupInfo::empty());
    let vec = vec![];
    pike_v_m.captures::<regex_automata::Input>(&mut cache, Input::new::<Vec<u8>>(&vec), &mut captures);
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_util_sparse_set_rs_197_32_96a160`).

## Output

```
thread 'main' (461437) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/sparse_set.rs:197:32:
index out of bounds: the len is 5 but the index is 8
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[regex_automata::util::primitives::StateID]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[regex_automata::util::primitives::StateID] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<regex_automata::util::primitives::StateID> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <alloc::vec::Vec<regex_automata::util::primitives::StateID> as core::ops::index::Index<regex_automata::util::primitives::StateID>>::index
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/primitives.rs:574:22
   7: <regex_automata::util::sparse_set::SparseSet>::contains
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/sparse_set.rs:197:32
   8: <regex_automata::util::sparse_set::SparseSet>::insert
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/sparse_set.rs:173:17
   9: <regex_automata::nfa::thompson::pikevm::PikeVM>::epsilon_closure_explore
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1683:26
  10: <regex_automata::nfa::thompson::pikevm::PikeVM>::epsilon_closure
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1631:26
  11: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1364:22
  12: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots_imp
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1146:29
  13: <regex_automata::nfa::thompson::pikevm::PikeVM>::search_slots
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1127:24
  14: <regex_automata::nfa::thompson::pikevm::PikeVM>::search
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:1027:24
  15: <regex_automata::nfa::thompson::pikevm::PikeVM>::captures::<regex_automata::util::search::Input>
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/pikevm.rs:865:14
  16: repro_regex_automata_src_util_sparse_set_rs_197_32_96a160::main
             at ./src/main.rs:13:14
  17: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
