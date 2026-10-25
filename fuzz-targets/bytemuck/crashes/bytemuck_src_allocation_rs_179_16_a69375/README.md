# bytemuck/src/allocation.rs:179:16

- crate: https://github.com/Lokathor/bytemuck @ ea3616110b0134e9991fdb66be8892dbc427c5ac
- kind: panic
- location: `bytemuck/src/allocation.rs:179:16`
- last API call: `bytemuck::allocation::zeroed_rc_slice[char]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: LayoutError
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
    let _ = bytemuck::allocation::zeroed_rc_slice::<char>(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_bytemuck_src_allocation_rs_179_16_a69375`).

## Output

```
thread 'main' (432732) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:2326:41:
called `Result::unwrap()` on an `Err` value: LayoutError
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <alloc::rc::Rc<[core::mem::maybe_uninit::MaybeUninit<char>]>>::allocate_for_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:2326:41
   5: <alloc::rc::Rc<[char]>>::new_uninit_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:1136:31
   6: bytemuck::allocation::zeroed_rc_slice::<char>
             at /home/rafal/.cargo/git/checkouts/bytemuck-a04c2e12817ee75f/ea36161/src/allocation.rs:179:16
   7: repro_bytemuck_src_allocation_rs_179_16_a69375::main
             at ./src/main.rs:2:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
