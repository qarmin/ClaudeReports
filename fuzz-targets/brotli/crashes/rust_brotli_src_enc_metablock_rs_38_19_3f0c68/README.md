# rust-brotli/src/enc/metablock.rs:38:19

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/metablock.rs:38:19`
- last API call: `brotli::enc::metablock::BrotliInitDistanceParams`
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
use brotli::enc::BrotliEncoderInitParams;
use brotli::enc::metablock::BrotliInitDistanceParams;

fn main() {
    let mut brotli_encoder_params = BrotliEncoderInitParams();
    BrotliInitDistanceParams(&mut brotli_encoder_params, 10u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_metablock_rs_38_19_3f0c68`).

## Output

```
thread 'main' (1253988) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/metablock.rs:38:19:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: brotli::enc::metablock::BrotliInitDistanceParams
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/metablock.rs:38:19
   4: repro_rust_brotli_src_enc_metablock_rs_38_19_3f0c68::main
             at ./src/main.rs:6:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
