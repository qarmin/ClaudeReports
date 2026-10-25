# out of memory in sketches_ddsketch::DDSketch::add_with_count

- crate: https://github.com/mheffner/rust-sketches-ddsketch @ 9c517a0996152f9f12a609d7b720d7cead01fede
- kind: out of memory
- last API call: `sketches_ddsketch::DDSketch::add_with_count`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling sketches_ddsketch::DDSketch::add_with_count
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
sketches-ddsketch = { git = "https://github.com/mheffner/rust-sketches-ddsketch", rev = "9c517a0996152f9f12a609d7b720d7cead01fede" }
```

`src/main.rs`:

```rust
use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let mut d_d_sketch = DDSketch::new(Config::new(0.063f64, 1000000000u32, 1.0f64));
    d_d_sketch.add(f64::MAX);
    d_d_sketch.add_with_count(f64::INFINITY, 1u64);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_sketches_ddsketch_ddsketch_add_with_count_28ea8a`).

## Output

```
memory allocation of 8000000000 bytes failed
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
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<u64>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<u64>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<u64>>::extend_with
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3714:14
  10: <alloc::vec::Vec<u64>>::resize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3575:18
  11: <sketches_ddsketch::store::Store>::extend_range
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/store.rs:89:27
  12: <sketches_ddsketch::store::Store>::get_index
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/store.rs:66:18
  13: <sketches_ddsketch::store::Store>::add_count
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/store.rs:50:24
  14: <sketches_ddsketch::ddsketch::DDSketch>::add_with_count
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/ddsketch.rs:91:24
  15: repro_out_of_memory_in_sketches_ddsketch_ddsketch_add_with_count_28ea8a::main
             at ./src/main.rs:7:16
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
