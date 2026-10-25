# rust-brotli/src/enc/command.rs:236:28

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/command.rs:236:28`
- last API call: `brotli::enc::command::Command::distance_index_and_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef" }
```

`src/main.rs`:

```rust
use brotli::enc::command::BrotliDistanceParams;
use brotli::enc::command::Command;

fn main() {
    let command = Command { insert_len_: 0u32, copy_len_: 0u32, dist_extra_: 0u32, cmd_prefix_: 0u16, dist_prefix_: 10000u16 };
    let _ = command.distance_index_and_offset(&BrotliDistanceParams { distance_postfix_bits: 1000000000u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 0usize });
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_command_rs_236_28_5831a9`).

## Output

```
thread 'main' (1253511) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/command.rs:236:28:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <brotli::enc::command::Command>::distance_index_and_offset
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/command.rs:236:28
   4: repro_rust_brotli_src_enc_command_rs_236_28_5831a9::main
             at ./src/main.rs:6:21
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
