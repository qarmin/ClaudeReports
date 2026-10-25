# ash/src/vk/native.rs:20:9

- crate: https://github.com/ash-rs/ash @ f4c2ca3e4f6b998d5254ad101a32f024d87cdec2
- kind: panic
- location: `ash/src/vk/native.rs:20:9`
- last API call: `ash::vk::native::__BindgenBitfieldUnit<[u8; 2]>::get`
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
use ash::vk::native::StdVideoH264SpsVuiFlags;

fn main() {
    let bindgen_bitfield_unit = StdVideoH264SpsVuiFlags::new_bitfield_1(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let _ = bindgen_bitfield_unit.get(1usize, 16u8);
}
```

Run with `cargo run` in this directory (package `repro_ash_src_vk_native_rs_20_9_48ffe5`).

## Output

```
thread 'main' (445438) panicked at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/native.rs:20:9:
assertion failed: index / 8 < self.storage.as_ref().len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <ash::vk::native::__BindgenBitfieldUnit<[u8; 2]>>::get_bit
             at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/native.rs:20:9
   4: <ash::vk::native::__BindgenBitfieldUnit<[u8; 2]>>::get
             at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/native.rs:55:21
   5: repro_ash_src_vk_native_rs_20_9_48ffe5::main
             at ./src/main.rs:5:35
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
