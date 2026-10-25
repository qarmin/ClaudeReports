# out of memory in snap::raw::Decoder::decompress_vec

- crate: https://github.com/BurntSushi/rust-snappy @ 29fcab53647bcd8cd1550c06bde0cd42777e5565
- kind: out of memory
- last API call: `snap::raw::Decoder::decompress_vec`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling snap::raw::Decoder::decompress_vec
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
snap = { git = "https://github.com/BurntSushi/rust-snappy", rev = "29fcab53647bcd8cd1550c06bde0cd42777e5565" }
```

`src/main.rs`:

```rust
use snap::raw::Decoder;

fn main() {
    let mut decoder = Decoder::new();
    let _ = decoder.decompress_vec(&vec![144u8, 216u8, 179u8, 191u8, 11u8]);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_snap_raw_decoder_decompress_vec_94bbe5`).

## Output

```
memory allocation of 3085757456 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <snap::decompress::Decoder>::decompress_vec
             at /home/rafal/.cargo/git/checkouts/rust-snappy-a35f52f4969c4a66/29fcab5/src/decompress.rs:106:23
  10: repro_out_of_memory_in_snap_raw_decoder_decompress_vec_94bbe5::main
             at ./src/main.rs:5:21
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
