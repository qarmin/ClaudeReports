# out of memory in fixedbitset::FixedBitSet::union_with

- crate: https://github.com/petgraph/fixedbitset @ 111fb2552130f6f4c06a4a1a2e0de7ea6e28b922
- kind: out of memory
- last API call: `fixedbitset::FixedBitSet::union_with`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling fixedbitset::FixedBitSet::union_with
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fixedbitset = { git = "https://github.com/petgraph/fixedbitset", rev = "111fb2552130f6f4c06a4a1a2e0de7ea6e28b922" }
```

`src/main.rs`:

```rust
use fixedbitset::FixedBitSet;

fn main() {
    let mut fixed_bit_set = FixedBitSet::default();
    fixed_bit_set.union_with(&FixedBitSet::with_capacity(1000000000000000000usize));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_fixedbitset_fixedbitset_union_with_d300ad`).

## Output

```
memory allocation of 125000000000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<fixedbitset::block::sse2::Block>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<fixedbitset::block::sse2::Block>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <fixedbitset::block::sse2::Block as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:14:21
   9: alloc::vec::from_elem::<fixedbitset::block::sse2::Block>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  10: <fixedbitset::FixedBitSet>::with_capacity
             at /home/rafal/.cargo/git/checkouts/fixedbitset-8ed885828547678f/111fb25/src/lib.rs:107:35
  11: repro_out_of_memory_in_fixedbitset_fixedbitset_union_with_d300ad::main
             at ./src/main.rs:5:31
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
