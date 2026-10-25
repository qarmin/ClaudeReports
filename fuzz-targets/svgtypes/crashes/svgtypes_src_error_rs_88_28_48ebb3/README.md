# svgtypes/src/error.rs:88:28

- crate: https://github.com/linebender/svgtypes @ 3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa
- kind: panic
- location: `svgtypes/src/error.rs:88:28`
- last API call: `svgtypes::Error::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
svgtypes = { git = "https://github.com/linebender/svgtypes", rev = "3ee4db5bcbfe8db817d87b55a3ae4f889bf2d3fa" }
```

`src/main.rs`:

```rust
use svgtypes::Error;

fn main() {
    let error = Error::InvalidString(vec![], 0usize);
    let _ = error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_svgtypes_src_error_rs_88_28_48ebb3`).

## Output

```
thread 'main' (489225) panicked at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/error.rs:88:28:
range start index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[alloc::string::String]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[alloc::string::String] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<alloc::string::String> as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <svgtypes::error::Error as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/svgtypes-da914f7dbc9e8d80/3ee4db5/src/error.rs:88:28
   7: <svgtypes::error::Error as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   8: <svgtypes::error::Error as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   9: repro_svgtypes_src_error_rs_88_28_48ebb3::main
             at ./src/main.rs:5:19
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
