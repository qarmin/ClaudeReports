# out of memory in rmp::encode::ByteBuf::with_capacity

- crate: https://github.com/3Hren/msgpack-rust @ cf880019f72e35a884659a9b29d9eb89b862670b
- kind: out of memory
- last API call: `rmp::encode::ByteBuf::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling rmp::encode::ByteBuf::with_capacity
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rmp = { git = "https://github.com/3Hren/msgpack-rust", rev = "cf880019f72e35a884659a9b29d9eb89b862670b" }
```

`src/main.rs`:

```rust
use rmp::encode::ByteBuf;

fn main() {
    let _ = ByteBuf::with_capacity(4294967296usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_rmp_encode_bytebuf_with_capacity_548983`).

## Output

```
memory allocation of 4294967296 bytes failed
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
   9: <rmp::encode::buffer::ByteBuf>::with_capacity
             at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/encode/buffer.rs:96:23
  10: repro_out_of_memory_in_rmp_encode_bytebuf_with_capacity_548983::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
