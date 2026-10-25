# out of memory in regex_automata::nfa::thompson::Compiler::build_from_hir

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: out of memory
- last API call: `regex_automata::nfa::thompson::Compiler::build_from_hir`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 13356205976874114556

## Message

```
process ended abnormally (out of memory) while calling regex_automata::nfa::thompson::Compiler::build_from_hir
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
use regex_automata::nfa::thompson::NFA;
use regex_syntax::hir::Hir;
use regex_syntax::hir::Repetition;

fn main() {
    let compiler = NFA::compiler();
    let _ = compiler.build_from_hir(&Hir::repetition(Repetition { min: 0u32, max: Some(100000000u32), greedy: false, sub: Box::new(regex_automata::util::syntax::parse("a").unwrap()) }));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_regex_automata_nfa_thompson_compiler_build__163560`).

## Output

```
memory allocation of 2147483648 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::grow_one
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:494:13
   6: <alloc::raw_vec::RawVec<regex_automata::nfa::thompson::builder::State>>::grow_one
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:188:29
   7: <alloc::vec::Vec<regex_automata::nfa::thompson::builder::State>>::push_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:22
   8: <alloc::vec::Vec<regex_automata::nfa::thompson::builder::State>>::push
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:996:22
   9: <regex_automata::nfa::thompson::builder::Builder>::add
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/builder.rs:1118:21
  10: <regex_automata::nfa::thompson::builder::Builder>::add_union_reverse
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/builder.rs:743:14
  11: <regex_automata::nfa::thompson::compiler::Compiler>::add_union_reverse
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1682:35
  12: <regex_automata::nfa::thompson::compiler::Compiler>::c_bounded
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1231:22
  13: <regex_automata::nfa::thompson::compiler::Compiler>::c_repetition
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1174:38
  14: <regex_automata::nfa::thompson::compiler::Compiler>::c
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1028:41
  15: <regex_automata::nfa::thompson::compiler::Compiler>::c_cap
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1157:26
  16: <regex_automata::nfa::thompson::compiler::Compiler>::compile::<&regex_syntax::hir::Hir>::{closure#1}
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1002:28
  17: <&mut <regex_automata::nfa::thompson::compiler::Compiler>::compile<&regex_syntax::hir::Hir>::{closure#1} as core::ops::function::FnOnce<(&&regex_syntax::hir::Hir,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
  18: <core::option::Option<&&regex_syntax::hir::Hir>>::map::<core::result::Result<regex_automata::nfa::thompson::compiler::ThompsonRef, regex_automata::nfa::thompson::error::BuildError>, &mut <regex_automata::nfa::thompson::compiler::Compiler>::compile<&regex_syntax::hir::Hir>::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
  19: <core::iter::adapters::map::Map<core::slice::iter::Iter<&regex_syntax::hir::Hir>, <regex_automata::nfa::thompson::compiler::Compiler>::compile<&regex_syntax::hir::Hir>::{closure#1}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
  20: <regex_automata::nfa::thompson::compiler::Compiler>::c_alt_iter::<core::iter::adapters::map::Map<core::slice::iter::Iter<&regex_syntax::hir::Hir>, <regex_automata::nfa::thompson::compiler::Compiler>::compile<&regex_syntax::hir::Hir>::{closure#1}>>
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1110:30
  21: <regex_automata::nfa::thompson::compiler::Compiler>::compile::<&regex_syntax::hir::Hir>
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:1000:29
  22: <regex_automata::nfa::thompson::compiler::Compiler>::build_many_from_hir::<&regex_syntax::hir::Hir>
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:903:14
  23: <regex_automata::nfa::thompson::compiler::Compiler>::build_from_hir
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/nfa/thompson/compiler.rs:859:14
  24: repro_out_of_memory_in_regex_automata_nfa_thompson_compiler_build__163560::main
             at ./src/main.rs:7:22
  25: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
