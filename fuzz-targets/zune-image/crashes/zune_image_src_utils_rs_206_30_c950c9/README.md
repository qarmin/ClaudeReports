# zune-image/src/utils.rs:206:30

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/utils.rs:206:30`
- last API call: `zune_image::utils::swizzle_channels[u8]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9638006153863004633

## Message

```
index out of bounds: the len is 0 but the index is 0
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
    let channel = Channel::new_with_bit_type(0usize, BitType::U8);
    let _ = zune_image::utils::swizzle_channels::<u8>(&[channel.clone(), channel.clone(), Channel::new_with_bit_type(0usize, BitType::U8), channel.clone(), channel.clone()], &mut vec![0u8, 80u8, 78u8, 71u8, 0u8]);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_utils_rs_206_30_c950c9`).

## Output

```
thread 'main' (269511) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/utils.rs:206:30:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: zune_image::utils::swizzle_channels::<u8>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/utils.rs:206:30
   4: repro_zune_image_src_utils_rs_206_30_c950c9::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
