# bzip2-rs/src/mem.rs:122:13

- crate: https://github.com/trifectatechfoundation/bzip2-rs @ 57f97219b90dac2ac6f0f62745e900dde0d48fde
- kind: panic
- location: `bzip2-rs/src/mem.rs:122:13`
- last API call: `bzip2::Compress::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: -2
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bzip2 = { git = "https://github.com/trifectatechfoundation/bzip2-rs", rev = "57f97219b90dac2ac6f0f62745e900dde0d48fde" }
```

`src/main.rs`:

```rust
use bzip2::Compress;
use bzip2::Compression;

fn main() {
    let compression = Compression::best();
    let _ = Compress::new(compression, 1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_bzip2_rs_src_mem_rs_122_13_e6057e`).

## Output

```
thread 'main' (1448277) panicked at /home/rafal/.cargo/git/checkouts/bzip2-rs-cf492a49d908a9a8/57f9721/src/mem.rs:122:13:
assertion `left == right` failed
  left: -2
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<i32, i32>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <bzip2::mem::Compress>::new
             at /home/rafal/.cargo/git/checkouts/bzip2-rs-cf492a49d908a9a8/57f9721/src/mem.rs:122:13
   5: repro_bzip2_rs_src_mem_rs_122_13_e6057e::main
             at ./src/main.rs:6:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
