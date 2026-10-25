# out of memory in widestring::Utf16String::reserve

- crate: https://github.com/VoidStarKat/widestring-rs @ 5c65b30447e5cf4beb824eb2d6464b63110bce76
- kind: out of memory
- last API call: `widestring::Utf16String::reserve`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling widestring::Utf16String::reserve
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
widestring = { git = "https://github.com/VoidStarKat/widestring-rs", rev = "5c65b30447e5cf4beb824eb2d6464b63110bce76" }
```

`src/main.rs`:

```rust
use widestring::Utf16String;

fn main() {
    let mut utf16_string = Utf16String::with_capacity(0usize);
    utf16_string.reserve(2147483648usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_widestring_utf16string_reserve_9b5f92`).

## Output

```
memory allocation of 4294967296 bytes failed
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
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<u16>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<u16>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <widestring::utfstring::Utf16String>::reserve
             at /home/rafal/.cargo/git/checkouts/widestring-rs-569ab19e7910d887/5c65b30/src/utfstring.rs:167:28
  10: repro_out_of_memory_in_widestring_utf16string_reserve_9b5f92::main
             at ./src/main.rs:5:18
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
