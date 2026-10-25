# out of memory in fax::VecWriter::with_capacity

- crate: https://github.com/pdf-rs/fax @ 8f26f4f63a54fb93529ffdedd9b5a584c3ed01af
- kind: out of memory
- last API call: `fax::VecWriter::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling fax::VecWriter::with_capacity
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fax = { git = "https://github.com/pdf-rs/fax", rev = "8f26f4f63a54fb93529ffdedd9b5a584c3ed01af" }
```

`src/main.rs`:

```rust
use fax::VecWriter;

fn main() {
    let _ = VecWriter::with_capacity(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_fax_vecwriter_with_capacity_ed4937`).

## Output

```
memory allocation of 1250000000000000000 bytes failed
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
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<u8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<u8>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <fax::VecWriter>::with_capacity
             at /home/rafal/.cargo/git/checkouts/fax-3c34a03a0b7cfe42/8f26f4f/src/lib.rs:100:19
  10: repro_out_of_memory_in_fax_vecwriter_with_capacity_ed4937::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
