# out of memory in tempfile::Builder::tempfile

- crate: https://github.com/Stebalien/tempfile @ 1c294cc29d2a48746637252a0e30b1bebc05b3e6
- kind: out of memory
- last API call: `tempfile::Builder::tempfile`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling tempfile::Builder::tempfile
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
    let _ = builder.rand_bytes(1000000000000000000usize);
    let _ = builder.tempfile();
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_tempfile_builder_tempfile_cb3d4d`).

## Output

```
memory allocation of 1000000000000000004 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<u8>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <std::sys::os_str::bytes::Buf>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/os_str/bytes.rs:106:22
  10: <std::ffi::os_str::OsString>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/ffi/os_str.rs:305:27
  11: tempfile::util::tmpname
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/util.rs:12:19
  12: tempfile::util::create_helper::<tempfile::file::NamedTempFile, <tempfile::Builder>::tempfile_in<std::path::PathBuf>::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/util.rs:67:30
  13: <tempfile::Builder>::tempfile_in::<std::path::PathBuf>
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/lib.rs:556:9
  14: <tempfile::Builder>::tempfile
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/lib.rs:527:14
  15: repro_out_of_memory_in_tempfile_builder_tempfile_cb3d4d::main
             at ./src/main.rs:6:21
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
