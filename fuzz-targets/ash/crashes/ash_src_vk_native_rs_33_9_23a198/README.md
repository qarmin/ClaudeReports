# ash/src/vk/native.rs:33:9

- crate: https://github.com/ash-rs/ash @ f4c2ca3e4f6b998d5254ad101a32f024d87cdec2
- kind: panic
- location: `ash/src/vk/native.rs:33:9`
- last API call: `ash::vk::native::__BindgenBitfieldUnit<[u8; 4]>::set_bit`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: index / 8 < self.storage.as_ref().len()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ash = { git = "https://github.com/ash-rs/ash", rev = "f4c2ca3e4f6b998d5254ad101a32f024d87cdec2" }
```

`src/main.rs`:

```rust
use ash::vk::native::StdVideoDecodeAV1PictureInfoFlags;

fn main() {
    let mut bindgen_bitfield_unit = StdVideoDecodeAV1PictureInfoFlags::new_bitfield_1(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    bindgen_bitfield_unit.set_bit(10000000000000000000usize, false);
}
```

Run with `cargo run` in this directory (package `repro_ash_src_vk_native_rs_33_9_23a198`).

## Output

```
thread 'main' (445547) panicked at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/native.rs:33:9:
assertion failed: index / 8 < self.storage.as_ref().len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <ash::vk::native::__BindgenBitfieldUnit<[u8; 4]>>::set_bit
             at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/native.rs:33:9
   4: repro_ash_src_vk_native_rs_33_9_23a198::main
             at ./src/main.rs:5:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
