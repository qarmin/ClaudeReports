# rust-brotli/src/enc/interface.rs:584:24

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/interface.rs:584:24`
- last API call: `brotli::SliceOffset::thaw`
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
use brotli::SliceOffset;

fn main() {
    let slice_offset = SliceOffset(1usize, 0u32);
    let vec = vec![];
    let _ = <brotli::SliceOffset as brotli::interface::Unfreezable>::thaw(&slice_offset, &vec);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_interface_rs_584_24_37d6d8`).

## Output

```
thread 'main' (1254857) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/interface.rs:584:24:
mid > len
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[u8]>::split_at
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1964:21
   3: <brotli::enc::interface::SliceOffset as brotli::enc::interface::Unfreezable>::thaw
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/interface.rs:584:24
   4: repro_rust_brotli_src_enc_interface_rs_584_24_37d6d8::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
