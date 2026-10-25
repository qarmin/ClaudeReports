# rust-brotli/src/enc/compress_fragment_two_pass.rs:409:23

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/compress_fragment_two_pass.rs:409:23`
- last API call: `brotli::enc::compress_fragment_two_pass::BrotliWriteBits`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 125000000000000000 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::compress_fragment_two_pass::BrotliWriteBits;

fn main() {
    BrotliWriteBits(0usize, 0u64, &mut 1000000000000000000usize, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_compress_fragment_two_pass_rs_409_23_7e25a6`).

## Output

```
thread 'main' (1255094) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/compress_fragment_two_pass.rs:409:23:
range start index 125000000000000000 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:558:13
   4: <[u8] as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: brotli::enc::compress_fragment_two_pass::BrotliWriteBits
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/compress_fragment_two_pass.rs:409:23
   6: repro_rust_brotli_src_enc_compress_fragment_two_pass_rs_409_23_7e25a6::main
             at ./src/main.rs:4:5
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
