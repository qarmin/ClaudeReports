# out of memory in bytemuck::allocation::zeroed_arc_slice[u8]

- crate: https://github.com/Lokathor/bytemuck @ ea3616110b0134e9991fdb66be8892dbc427c5ac
- kind: out of memory
- last API call: `bytemuck::allocation::zeroed_arc_slice[u8]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling bytemuck::allocation::zeroed_arc_slice[u8]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bytemuck = { git = "https://github.com/Lokathor/bytemuck", rev = "ea3616110b0134e9991fdb66be8892dbc427c5ac", features = ["aarch64_simd", "align_offset", "alloc_uninit", "avx512_simd", "bytemuck_derive", "const_zeroed", "derive", "extern_crate_alloc", "extern_crate_std", "impl_core_error", "latest_stable_rust", "min_const_generics", "must_cast", "must_cast_extra", "nightly_stdsimd", "pod_saturating", "rustversion", "track_caller", "transparentwrapper_extra", "unsound_ptr_pod_impl", "wasm_simd", "zeroable_atomics", "zeroable_maybe_uninit", "zeroable_unwind_fn"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = bytemuck::allocation::zeroed_arc_slice::<u8>(2147483648usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_bytemuck_allocation_zeroed_arc_slice_u8_d98164`).

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
   4: <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_layout::<<alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#0}, <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#1}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2191:55
   5: <core::result::Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>>::unwrap_or_else::<<alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_layout<<alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#0}, <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#1}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1621:23
   6: <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_layout::<<alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#0}, <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2191:36
   7: <alloc::sync::Arc<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::allocate_for_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:2276:13
   8: <alloc::sync::Arc<[u8]>>::new_uninit_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1290:32
   9: bytemuck::allocation::zeroed_arc_slice::<u8>
             at /home/rafal/.cargo/git/checkouts/bytemuck-a04c2e12817ee75f/ea36161/src/allocation.rs:163:17
  10: repro_out_of_memory_in_bytemuck_allocation_zeroed_arc_slice_u8_d98164::main
             at ./src/main.rs:2:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
