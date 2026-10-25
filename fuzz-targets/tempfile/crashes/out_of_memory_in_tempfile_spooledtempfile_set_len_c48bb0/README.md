# out of memory in tempfile::SpooledTempFile::set_len

- crate: https://github.com/Stebalien/tempfile @ 1c294cc29d2a48746637252a0e30b1bebc05b3e6
- kind: out of memory
- last API call: `tempfile::SpooledTempFile::set_len`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling tempfile::SpooledTempFile::set_len
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tempfile = { git = "https://github.com/Stebalien/tempfile", rev = "1c294cc29d2a48746637252a0e30b1bebc05b3e6" }
```

`src/main.rs`:

```rust
fn main() {
    let mut spooled_temp_file = tempfile::spooled_tempfile(1000000000000000000usize);
    let _ = spooled_temp_file.set_len(1000000000000000000u64);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_tempfile_spooledtempfile_set_len_c48bb0`).

## Output

```
memory allocation of 1000000000000000000 bytes failed
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
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::vec::Vec<u8>>::extend_with
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3714:14
   7: <alloc::vec::Vec<u8>>::resize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3575:18
   8: <tempfile::spooled::SpooledTempFile>::set_len
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/spooled.rs:140:34
   9: repro_out_of_memory_in_tempfile_spooledtempfile_set_len_c48bb0::main
             at ./src/main.rs:3:31
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
