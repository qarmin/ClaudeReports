# rmp/src/decode/str.rs:151:30

- crate: https://github.com/3Hren/msgpack-rust @ cf880019f72e35a884659a9b29d9eb89b862670b
- kind: panic
- location: `rmp/src/decode/str.rs:151:30`
- last API call: `rmp::decode::read_str_ref`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 25 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rmp = { git = "https://github.com/3Hren/msgpack-rust", rev = "cf880019f72e35a884659a9b29d9eb89b862670b" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = rmp::decode::read_str_ref(&vec![185u8]);
}
```

Run with `cargo run` in this directory (package `repro_rmp_src_decode_str_rs_151_30_9d7927`).

## Output

```
thread 'main' (447124) panicked at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/str.rs:151:30:
range end index 25 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:503:23
   5: <[u8] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: rmp::decode::str::read_str_ref
             at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/str.rs:151:30
   7: repro_rmp_src_decode_str_rs_151_30_9d7927::main
             at ./src/main.rs:2:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
