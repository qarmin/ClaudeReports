# petgraph/src/graph6/graph6_decoder.rs:58:45

- crate: https://github.com/petgraph/petgraph @ b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495
- kind: panic
- location: `petgraph/src/graph6/graph6_decoder.rs:58:45`
- last API call: `petgraph::graph6::from_graph6_representation[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 3 out of range for slice of length 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
petgraph = { git = "https://github.com/petgraph/petgraph", rev = "b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = petgraph::graph6::from_graph6_representation::<u16>("~".to_string());
}
```

Run with `cargo run` in this directory (package `repro_petgraph_src_graph6_graph6_decoder_rs_58_45_f6863f`).

## Output

```
thread 'main' (1350538) panicked at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:58:45:
range end index 3 out of range for slice of length 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeInclusive<usize> as core::slice::index::SliceIndex<[usize]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:689:9
   4: <[usize] as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<usize> as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:58:45
   7: petgraph::graph6::graph6_decoder::from_graph6_representation::<u16>
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:33:9
   8: repro_petgraph_src_graph6_graph6_decoder_rs_58_45_f6863f::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
