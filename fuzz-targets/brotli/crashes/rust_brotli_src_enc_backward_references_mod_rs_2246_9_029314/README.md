# rust-brotli/src/enc/backward_references/mod.rs:2246:9

- crate: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef
- kind: panic
- location: `rust-brotli/src/enc/backward_references/mod.rs:2246:9`
- last API call: `brotli::enc::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator>::StitchToPreviousBlock`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
UNINTIALIZED
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
brotli = { git = "https://github.com/dropbox/rust-brotli", rev = "6518a4eff2f9b93a5e81555bdb10c2979b3057ef", features = ["alloc-stdlib", "billing", "disable-timer", "disallow_large_window_size", "ffi-api", "floating_point_context_mixing", "no-stdlib-ffi-binding", "pass-through-ffi-panics", "portable-float", "safe", "seccomp", "sha2", "std", "validation", "vector_scratch_space"] }
```

`src/main.rs`:

```rust
fn main() {
    let mut union_hasher = <brotli::enc::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator> as ::core::default::Default>::default();
    <brotli::enc::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator> as brotli::enc::backward_references::AnyHasher>::StitchToPreviousBlock(&mut union_hasher, 0usize, 0usize, &vec![], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_rust_brotli_src_enc_backward_references_mod_rs_2246_9_029314`).

## Output

```
thread 'main' (1254683) panicked at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/mod.rs:2246:9:
UNINTIALIZED
stack backtrace:
   0: std::panicking::begin_panic::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:751:5
   1: <brotli::enc::backward_references::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator> as brotli::enc::backward_references::AnyHasher>::StitchToPreviousBlock
             at /home/rafal/.cargo/git/checkouts/rust-brotli-e77e19ae2d801897/6518a4e/src/enc/backward_references/mod.rs:2193:34
   2: repro_rust_brotli_src_enc_backward_references_mod_rs_2246_9_029314::main
             at ./src/main.rs:3:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
