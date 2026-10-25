# block-padding/src/lib.rs:255:9

- crate: https://github.com/RustCrypto/utils @ 861955eaccf7a7ebe9a214667b8df221971a2e2f
- kind: panic
- location: `block-padding/src/lib.rs:255:9`
- last API call: `block_padding::AnsiX923::raw_pad`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
block size is too big for ANSI X9.23
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
cpufeatures = { git = "https://github.com/RustCrypto/utils", rev = "861955eaccf7a7ebe9a214667b8df221971a2e2f" }
block-padding = { git = "https://github.com/RustCrypto/utils", rev = "861955eaccf7a7ebe9a214667b8df221971a2e2f" }
```

`src/main.rs`:

```rust
fn main() {
    <block_padding::AnsiX923 as block_padding::Padding>::raw_pad(&mut include_bytes!("data_0.bin").to_vec(), 0usize);
}
```

Run with `cargo run` in this directory (package `repro_block_padding_src_lib_rs_255_9_e0a8ae`).

## Output

```
thread 'main' (2086966) panicked at /home/rafal/.cargo/git/checkouts/utils-f40fa06513bf42b3/861955e/block-padding/src/lib.rs:255:9:
block size is too big for ANSI X9.23
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <block_padding::AnsiX923 as block_padding::Padding>::raw_pad
             at /home/rafal/.cargo/git/checkouts/utils-f40fa06513bf42b3/861955e/block-padding/src/lib.rs:255:9
   3: repro_block_padding_src_lib_rs_255_9_e0a8ae::main
             at ./src/main.rs:2:5
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
