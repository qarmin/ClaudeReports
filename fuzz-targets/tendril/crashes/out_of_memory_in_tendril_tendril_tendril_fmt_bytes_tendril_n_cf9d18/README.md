# out of memory in tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>::with_capacity

- crate: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45
- kind: out of memory
- last API call: `tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>::with_capacity
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tendril = { git = "https://github.com/servo/html5ever", rev = "7760920edff08e6dfd4b62affee39d8084e9dc45" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(1537130801u32);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_cf9d18`).

## Output

```
memory allocation of 2147483664 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::reserve_exact
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:708:13
   6: <alloc::raw_vec::RawVec<tendril::tendril::Header<tendril::tendril::NonAtomic>>>::reserve_exact
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:374:29
   7: <alloc::vec::Vec<tendril::tendril::Header<tendril::tendril::NonAtomic>>>::reserve_exact
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1491:18
   8: <tendril::buf32::Buf32<tendril::tendril::Header<tendril::tendril::NonAtomic>>>::grow
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/buf32.rs:97:13
   9: <tendril::tendril::Tendril<tendril::fmt::Bytes>>::make_owned_with_capacity
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:1077:13
  10: <tendril::tendril::Tendril<tendril::fmt::Bytes>>::with_capacity
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:574:19
  11: repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_cf9d18::main
             at ./src/main.rs:2:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
