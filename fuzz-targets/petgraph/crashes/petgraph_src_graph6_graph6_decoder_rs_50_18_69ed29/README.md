# petgraph/src/graph6/graph6_decoder.rs:50:18

- crate: https://github.com/petgraph/petgraph @ b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495
- kind: panic
- location: `petgraph/src/graph6/graph6_decoder.rs:50:18`
- last API call: `petgraph::graph6::from_graph6_representation[usize]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
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
    let _ = petgraph::graph6::from_graph6_representation::<usize>("(".to_string());
}
```

Run with `cargo run` in this directory (package `repro_petgraph_src_graph6_graph6_decoder_rs_50_18_69ed29`).

## Output

```
thread 'main' (1350369) panicked at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:50:18:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:50:18
   4: <&mut petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0} as core::ops::function::FnOnce<(char,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   5: <core::option::Option<char>>::map::<usize, &mut petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
   6: <core::iter::adapters::map::Map<core::str::iter::Chars, petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   7: <alloc::vec::Vec<usize> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<usize, core::iter::adapters::map::Map<core::str::iter::Chars, petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:24:41
   8: <alloc::vec::Vec<usize> as alloc::vec::spec_from_iter::SpecFromIter<usize, core::iter::adapters::map::Map<core::str::iter::Chars, petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
   9: <alloc::vec::Vec<usize> as core::iter::traits::collect::FromIterator<usize>>::from_iter::<core::iter::adapters::map::Map<core::str::iter::Chars, petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  10: <core::iter::adapters::map::Map<core::str::iter::Chars, petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0}> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<usize>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  11: petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:51:10
  12: petgraph::graph6::graph6_decoder::from_graph6_representation::<usize>
             at /home/rafal/.cargo/git/checkouts/petgraph-15f5ad8cf9204101/b3eaf3a/crates/petgraph/src/graph6/graph6_decoder.rs:33:9
  13: repro_petgraph_src_graph6_graph6_decoder_rs_50_18_69ed29::main
             at ./src/main.rs:2:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
