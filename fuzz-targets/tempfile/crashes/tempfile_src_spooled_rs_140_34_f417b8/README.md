# tempfile/src/spooled.rs:140:34

- crate: https://github.com/Stebalien/tempfile @ 1c294cc29d2a48746637252a0e30b1bebc05b3e6
- kind: panic
- location: `tempfile/src/spooled.rs:140:34`
- last API call: `tempfile::SpooledTempFile::set_len`
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
use tempfile::SpooledTempFile;

fn main() {
    let mut spooled_temp_file = SpooledTempFile::new(usize::MAX);
    let _ = spooled_temp_file.set_len(u64::MAX);
}
```

Run with `cargo run` in this directory (package `repro_tempfile_src_spooled_rs_140_34_f417b8`).

## Output

```
thread 'main' (469852) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   4: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   5: <alloc::vec::Vec<u8>>::extend_with
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3714:14
   6: <alloc::vec::Vec<u8>>::resize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3575:18
   7: <tempfile::spooled::SpooledTempFile>::set_len
             at /home/rafal/.cargo/git/checkouts/tempfile-22f697c78bc9a128/1c294cc/src/spooled.rs:140:34
   8: repro_tempfile_src_spooled_rs_140_34_f417b8::main
             at ./src/main.rs:5:31
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
