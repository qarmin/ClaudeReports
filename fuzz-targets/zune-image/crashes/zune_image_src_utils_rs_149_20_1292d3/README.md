# zune-image/src/utils.rs:149:20

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/utils.rs:149:20`
- last API call: `zune_image::utils::swizzle_channels[u8]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1885574460529280156

## Message

```
copy_from_slice: source slice length (0) does not match destination slice length (1)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;

fn main() {
    let _ = zune_image::utils::swizzle_channels::<u8>(&[Channel::new_with_bit_type(0usize, BitType::U8)], &mut vec![0u8]);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_utils_rs_149_20_1292d3`).

## Output

```
thread 'main' (264653) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/utils.rs:149:20:
copy_from_slice: source slice length (0) does not match destination slice length (1)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::copy_from_slice_impl::len_mismatch_fail::do_panic::runtime
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:172:21
   3: core::slice::copy_from_slice_impl::len_mismatch_fail::do_panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
   4: core::slice::copy_from_slice_impl::len_mismatch_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   5: core::slice::copy_from_slice_impl::<u8>
   6: <[u8]>::copy_from_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:4344:18
   7: zune_image::utils::swizzle_channels::<u8>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/utils.rs:149:20
   8: repro_zune_image_src_utils_rs_149_20_1292d3::main
             at ./src/main.rs:5:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
