# out of memory in bytemuck::allocation::zeroed_rc_slice[bool]

- crate: https://github.com/Lokathor/bytemuck @ ea3616110b0134e9991fdb66be8892dbc427c5ac
- kind: out of memory
- last API call: `bytemuck::allocation::zeroed_rc_slice[bool]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling bytemuck::allocation::zeroed_rc_slice[bool]
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
    let _ = bytemuck::allocation::zeroed_rc_slice::<bool>(1000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_bytemuck_allocation_zeroed_rc_slice_bool_09f0ae`).

## Output

```
memory allocation of 1000000000000000016 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_layout::<<alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#0}, <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#1}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:2249:37
   5: <core::result::Result<*mut alloc::rc::RcInner<[core::mem::maybe_uninit::MaybeUninit<bool>]>, core::alloc::AllocError>>::unwrap_or_else::<<alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_layout<<alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#0}, <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#1}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1621:23
   6: <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_layout::<<alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#0}, <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:2249:18
   7: <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<bool>]>>::allocate_for_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:2325:13
   8: <alloc::rc::Rc<[bool]>>::new_uninit_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:1136:31
   9: bytemuck::allocation::zeroed_rc_slice::<bool>
             at /home/rafal/.cargo/git/checkouts/bytemuck-a04c2e12817ee75f/ea36161/src/allocation.rs:179:16
  10: repro_out_of_memory_in_bytemuck_allocation_zeroed_rc_slice_bool_09f0ae::main
             at ./src/main.rs:2:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
