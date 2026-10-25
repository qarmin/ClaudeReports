# rust-brotli/src/enc/backward_references/hq.rs:136:17

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/backward_references/hq.rs:136:17`
- last API call: `brotli::enc::backward_references_hq::BrotliZopfliCreateCommands`
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
use brotli::enc::BrotliEncoderParams;
use brotli::enc::ZopfliNode;
use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::backward_references::BrotliHasherParams;
use brotli::enc::backward_references_hq::BrotliZopfliCreateCommands;
use brotli::enc::command::BrotliDistanceParams;

fn main() {
    BrotliZopfliCreateCommands(0usize, 0usize, 0usize, &[ZopfliNode::default()], &mut vec![], &mut 0usize, &BrotliEncoderParams { dist: BrotliDistanceParams { distance_postfix_bits: 0u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 1usize }, mode: BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR, quality: 0i32, q9_5: false, lgwin: 0i32, lgblock: 0i32, size_hint: 0usize, disable_literal_context_modeling: 0i32, hasher: BrotliHasherParams { type_: 0i32, bucket_bits: 0i32, block_bits: 0i32, hash_len: 0i32, num_last_distances_to_check: 0i32, literal_byte_score: 0i32 }, log_meta_block: false, stride_detection_quality: 0u8, high_entropy_detection_quality: 0u8, cdf_adaptation_detection: 1u8, prior_bitmask_detection: 0u8, literal_adaptation: [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (1u16, 0u16)], large_window: false, avoid_distance_prefix_search: false, byte_align: false, bare_stream: false, catable: false, use_dictionary: false, appendable: false, magic_number: false, favor_cpu_efficiency: false }, &mut vec![], &mut 0usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_backward_references_hq_rs_136_17_7d23c0`).

## Output

```
thread 'main' (1253608) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/hq.rs:136:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: brotli::enc::backward_references::hq::BrotliZopfliCreateCommands
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/hq.rs:136:17
   4: repro_rust_brotli_src_enc_backward_references_hq_rs_136_17_7d23c0::main
             at ./src/main.rs:9:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
