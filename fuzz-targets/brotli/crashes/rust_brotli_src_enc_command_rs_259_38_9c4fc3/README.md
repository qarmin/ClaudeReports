# rust-brotli/src/enc/command.rs:259:38

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/command.rs:259:38`
- last API call: `brotli::enc::command::RecomputeDistancePrefixes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
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
use brotli::enc::command::RecomputeDistancePrefixes;

fn main() {
    RecomputeDistancePrefixes(&mut vec![], 1usize, 0u32, 1u32, &BrotliDistanceParams { distance_postfix_bits: 1u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 1usize });
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_command_rs_259_38_9c4fc3`).

## Output

```
thread 'main' (1254034) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/command.rs:259:38:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::command::RecomputeDistancePrefixes
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/command.rs:259:38
   4: repro_rust_brotli_src_enc_command_rs_259_38_9c4fc3::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
