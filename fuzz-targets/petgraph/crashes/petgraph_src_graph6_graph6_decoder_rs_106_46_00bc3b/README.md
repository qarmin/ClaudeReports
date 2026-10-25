# petgraph/src/graph6/graph6_decoder.rs:106:46

- crate: https://github.com/petgraph/petgraph @ b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495
- kind: panic
- location: `petgraph/src/graph6/graph6_decoder.rs:106:46`
- last API call: `petgraph::graph6::from_graph6_representation[u32]`
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
petgraph = { git = "https://github.com/petgraph/petgraph", rev = "b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = petgraph::graph6::from_graph6_representation::<u32>("a".to_string());
}
```

Run with `cargo run` in this directory (package `repro_petgraph_src_graph6_graph6_decoder_rs_106_46_00bc3b`).

## Output

```
thread 'main' (1350465) panicked at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:106:46:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[u8] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<u8> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: petgraph::graph6::graph6_decoder::get_edges::<u32>
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:106:46
   7: petgraph::graph6::graph6_decoder::from_graph6_representation::<u32>
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:39:17
   8: repro_petgraph_src_graph6_graph6_decoder_rs_106_46_00bc3b::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
