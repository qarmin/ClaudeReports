# tendril/src/utf8_decode.rs:95:45

- crate: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45
- kind: panic
- location: `tendril/src/utf8_decode.rs:95:45`
- last API call: `tendril::IncompleteUtf8::try_to_complete_codepoint`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 10 out of range for slice of length 4
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tendril = { git = "https://github.com/servo/html5ever", rev = "7760920edff08e6dfd4b62affee39d8084e9dc45" }
```

`src/main.rs`:

```rust
use tendril::IncompleteUtf8;

fn main() {
    let mut incomplete_utf8 = IncompleteUtf8 { buffer: [0u8, 0u8, 1u8, 0u8], buffer_len: 10u8 };
    let _ = incomplete_utf8.try_to_complete_codepoint(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_tendril_src_utf8_decode_rs_95_45_508fe0`).

## Output

```
thread 'main' (358080) panicked at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:95:45:
range start index 10 out of range for slice of length 4
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:558:13
   4: <[u8] as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <[u8; 4] as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:403:9
   6: <tendril::utf8_decode::IncompleteUtf8>::try_complete_offsets
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:95:45
   7: <tendril::utf8_decode::IncompleteUtf8>::try_to_complete_codepoint
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/utf8_decode.rs:141:50
   8: repro_tendril_src_utf8_decode_rs_95_45_508fe0::main
             at ./src/main.rs:5:29
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
