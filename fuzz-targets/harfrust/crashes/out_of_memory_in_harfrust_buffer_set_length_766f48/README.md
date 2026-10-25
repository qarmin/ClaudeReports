# out of memory in harfrust::Buffer::set_length

- crate: https://github.com/harfbuzz/harfrust @ facb5efa7f301e2a27cadb3a43dfde93bba3f1f2
- kind: out of memory
- last API call: `harfrust::Buffer::set_length`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling harfrust::Buffer::set_length
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
harfrust = { git = "https://github.com/harfbuzz/harfrust", rev = "facb5efa7f301e2a27cadb3a43dfde93bba3f1f2" }
```

`src/main.rs`:

```rust
use harfrust::Buffer;

fn main() {
    let mut buffer = Buffer::new();
    let _ = buffer.set_length(100000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_harfrust_buffer_set_length_766f48`).

## Output

```
memory allocation of 2000000000 bytes failed
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
   7: <alloc::raw_vec::RawVec<harfrust::buffer::GlyphPosition>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<harfrust::buffer::GlyphPosition>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<harfrust::buffer::GlyphPosition>>::extend_with
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3714:14
  10: <alloc::vec::Vec<harfrust::buffer::GlyphPosition>>::resize
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3575:18
  11: <harfrust::buffer::Buffer>::enlarge
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/buffer.rs:2093:18
  12: <harfrust::buffer::Buffer>::ensure
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/buffer.rs:2081:18
  13: <harfrust::buffer::Buffer>::set_length
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/buffer.rs:657:18
  14: repro_out_of_memory_in_harfrust_buffer_set_length_766f48::main
             at ./src/main.rs:5:20
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
