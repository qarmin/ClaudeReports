# regex-automata/src/util/wire.rs:877:12

- crate: https://github.com/rust-lang/regex @ 72d650cb0a880a01ab6dc2137c0888e8f89740f7
- kind: panic
- location: `regex-automata/src/util/wire.rs:877:12`
- last API call: `regex_automata::dfa::sparse::DFA<Vec<u8>>::write_to_native_endian`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 4 out of range for slice of length 0
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
fn main() {
    let d_f_a = <regex_automata::dfa::sparse::DFA<Vec<u8>>>::always_match().unwrap();
    let _ = d_f_a.write_to_native_endian(&mut include_bytes!("data_0.bin").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_regex_automata_src_util_wire_rs_877_12_3e8a02`).

## Output

```
thread 'main' (461503) panicked at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/wire.rs:877:12:
range end index 4 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:427:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:509:23
   5: <[u8] as core::ops::index::IndexMut<core::ops::range::RangeTo<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   6: <regex_automata::util::wire::LE as regex_automata::util::wire::Endian>::write_u32
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/util/wire.rs:877:12
   7: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>>>::write_to::<regex_automata::util::wire::LE>
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:796:13
   8: <regex_automata::dfa::sparse::DFA<alloc::vec::Vec<u8>>>::write_to_native_endian
             at /home/rafal/.cargo/git/checkouts/regex-5e22c1b478b63697/72d650c/regex-automata/src/dfa/sparse.rs:781:14
   9: repro_regex_automata_src_util_wire_rs_877_12_3e8a02::main
             at ./src/main.rs:3:19
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
