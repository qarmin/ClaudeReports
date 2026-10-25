# widestring-rs/src/utfstring.rs:88:28

- crate: https://github.com/VoidStarKat/widestring-rs @ 5c65b30447e5cf4beb824eb2d6464b63110bce76
- kind: panic
- location: `widestring-rs/src/utfstring.rs:88:28`
- last API call: `widestring::Utf16String::with_capacity`
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
widestring = { git = "https://github.com/VoidStarKat/widestring-rs", rev = "5c65b30447e5cf4beb824eb2d6464b63110bce76" }
```

`src/main.rs`:

```rust
use widestring::Utf16String;

fn main() {
    let _ = Utf16String::with_capacity(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_widestring_rs_src_utfstring_rs_88_28_9e12ba`).

## Output

```
thread 'main' (1253246) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   5: <alloc::raw_vec::RawVec<u16>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<u16>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<u16>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <widestring::utfstring::Utf16String>::with_capacity
             at /home/rafal/.cargo/git/checkouts/widestring-rs-569ab19e7910d887/5c65b30/src/utfstring.rs:88:28
   9: repro_widestring_rs_src_utfstring_rs_88_28_9e12ba::main
             at ./src/main.rs:4:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
