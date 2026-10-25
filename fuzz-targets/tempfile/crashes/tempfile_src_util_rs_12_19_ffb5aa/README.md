# tempfile/src/util.rs:12:19

- crate: https://github.com/Stebalien/tempfile @ 1c294cc29d2a48746637252a0e30b1bebc05b3e6
- kind: panic
- location: `tempfile/src/util.rs:12:19`
- last API call: `tempfile::Builder::tempfile`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tempfile = { git = "https://github.com/Stebalien/tempfile", rev = "1c294cc29d2a48746637252a0e30b1bebc05b3e6" }
```

`src/main.rs`:

```rust
use tempfile::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.rand_bytes(10000000000000000000usize);
    let _ = builder.tempfile();
}
```

Run with `cargo run` in this directory (package `repro_tempfile_src_util_rs_12_19_ffb5aa`).

## Output

```
thread 'main' (469606) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   5: <alloc::raw_vec::RawVec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<u8>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <std::sys::os_str::bytes::Buf>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/os_str/bytes.rs:106:22
   9: <std::ffi::os_str::OsString>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/ffi/os_str.rs:305:27
  10: tempfile::util::tmpname
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/util.rs:12:19
  11: tempfile::util::create_helper::<tempfile::file::NamedTempFile, <tempfile::Builder>::tempfile_in<std::path::PathBuf>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/util.rs:67:30
  12: <tempfile::Builder>::tempfile_in::<std::path::PathBuf>
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/lib.rs:556:9
  13: <tempfile::Builder>::tempfile
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/lib.rs:527:14
  14: repro_tempfile_src_util_rs_12_19_ffb5aa::main
             at ./src/main.rs:6:21
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
