# rust-brotli/src/enc/static_dict.rs:46:31

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/static_dict.rs:46:31`
- last API call: `brotli::enc::backward_references::H6Sub::load_and_mix_word`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
mid > len
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::backward_references::H6Sub;

fn main() {
    let _ = <brotli::enc::backward_references::H6Sub as brotli::enc::backward_references::AdvHashSpecialization>::load_and_mix_word(&H6Sub { hash_mask: 0u64, hash_shift_: 0i32, bucket_size_: 0u32, block_mask_: 0u32, block_bits_: 0i32 }, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_static_dict_rs_46_31_894491`).

## Output

```
thread 'main' (1253554) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/static_dict.rs:46:31:
mid > len
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[u8]>::split_at
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1964:21
   3: brotli::enc::static_dict::BROTLI_UNALIGNED_LOAD64
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/static_dict.rs:46:31
   4: <brotli::enc::backward_references::H6Sub as brotli::enc::backward_references::AdvHashSpecialization>::load_and_mix_word
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/mod.rs:1139:10
   5: repro_rust_brotli_src_enc_static_dict_rs_46_31_894491::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
