# widestring-rs/src/ustring.rs:147:28

- crate: https://github.com/VoidStarKat/widestring-rs @ 5c65b30447e5cf4beb824eb2d6464b63110bce76
- kind: panic
- location: `widestring-rs/src/ustring.rs:147:28`
- last API call: `widestring::U32String::reserve`
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
use widestring::U32String;

fn main() {
    let mut u32_string = U32String::from_chars(vec![]);
    u32_string.reserve(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_widestring_rs_src_ustring_rs_147_28_ef7a62`).

## Output

```
thread 'main' (1253464) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   5: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   6: <alloc::raw_vec::RawVec<u32>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   7: <alloc::vec::Vec<u32>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   8: <widestring::ustring::U32String>::reserve
             at /home/rafal/.cargo/git/checkouts/widestring-rs-569ab19e7910d887/5c65b30/src/ustring.rs:147:28
   9: repro_widestring_rs_src_ustring_rs_147_28_ef7a62::main
             at ./src/main.rs:5:16
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
