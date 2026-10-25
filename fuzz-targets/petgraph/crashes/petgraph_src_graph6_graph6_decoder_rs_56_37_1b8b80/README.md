# petgraph/src/graph6/graph6_decoder.rs:56:37

- crate: https://github.com/petgraph/petgraph @ b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495
- kind: panic
- location: `petgraph/src/graph6/graph6_decoder.rs:56:37`
- last API call: `petgraph::graph6::from_graph6_representation[usize]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
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
    let _ = petgraph::graph6::from_graph6_representation::<usize>("".to_string());
}
```

Run with `cargo run` in this directory (package `repro_petgraph_src_graph6_graph6_decoder_rs_56_37_1b8b80`).

## Output

```
thread 'main' (1350417) panicked at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:56:37:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&usize>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:56:37
   6: petgraph::graph6::graph6_decoder::from_graph6_representation::<usize>
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:33:9
   7: repro_petgraph_src_graph6_graph6_decoder_rs_56_37_1b8b80::main
             at ./src/main.rs:2:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
