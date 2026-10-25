# rust-brotli/src/enc/backward_references/hq.rs:63:9

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/backward_references/hq.rs:63:9`
- last API call: `brotli::enc::backward_references_hq::BrotliInitZopfliNodes`
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
use brotli::enc::backward_references_hq::BrotliInitZopfliNodes;
use brotli::enc::constants::BROTLI_NUM_LITERAL_SYMBOLS;

fn main() {
    let usize_v = BROTLI_NUM_LITERAL_SYMBOLS;
    BrotliInitZopfliNodes(&mut vec![], usize_v);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_backward_references_hq_rs_63_9_b95556`).

## Output

```
thread 'main' (1255388) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/hq.rs:63:9:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::backward_references::hq::BrotliInitZopfliNodes
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/hq.rs:63:9
   4: repro_rust_brotli_src_enc_backward_references_hq_rs_63_9_b95556::main
             at ./src/main.rs:6:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
