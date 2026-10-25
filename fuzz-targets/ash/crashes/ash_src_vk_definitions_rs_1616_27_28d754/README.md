# ash/src/vk/definitions.rs:1616:27

- crate: https://github.com/ash-rs/ash @ f4c2ca3e4f6b998d5254ad101a32f024d87cdec2
- kind: panic
- location: `ash/src/vk/definitions.rs:1616:27`
- last API call: `ash::vk::PhysicalDeviceMemoryProperties::memory_heaps_as_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 4294967293 out of range for slice of length 16
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ash = { git = "https://github.com/ash-rs/ash", rev = "f4c2ca3e4f6b998d5254ad101a32f024d87cdec2" }
```

`src/main.rs`:

```rust
use ash::vk::MemoryHeap;
use ash::vk::MemoryHeapFlags;
use ash::vk::MemoryPropertyFlags;
use ash::vk::MemoryType;
use ash::vk::PhysicalDeviceMemoryProperties;
use ash::vk::PipelineLayoutCreateFlags;

fn main() {
    let pipeline_layout_create_flags = <ash::vk::PipelineLayoutCreateFlags as ::core::ops::BitOr<ash::vk::PipelineLayoutCreateFlags>>::bitor(PipelineLayoutCreateFlags::from_raw(0u32), <ash::vk::PipelineLayoutCreateFlags as ::core::ops::Not>::not(PipelineLayoutCreateFlags::INDEPENDENT_SETS_EXT));
    let u32_v = pipeline_layout_create_flags.as_raw();
    let physical_device_memory_properties = PhysicalDeviceMemoryProperties { memory_type_count: 0u32, memory_types: [MemoryType { property_flags: MemoryPropertyFlags::from_raw(1u32), heap_index: 0u32 }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }, MemoryType { property_flags: MemoryPropertyFlags::empty(), heap_index: u32_v }], memory_heap_count: u32_v, memory_heaps: [MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }, MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }] };
    let _ = physical_device_memory_properties.memory_heaps_as_slice();
}
```

Run with `cargo run` in this directory (package `repro_ash_src_vk_definitions_rs_1616_27_28d754`).

## Output

```
thread 'main' (1345496) panicked at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/definitions.rs:1616:27:
range end index 4294967293 out of range for slice of length 16
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[ash::vk::definitions::MemoryHeap]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[ash::vk::definitions::MemoryHeap]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:503:23
   5: <[ash::vk::definitions::MemoryHeap] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: <[ash::vk::definitions::MemoryHeap; 16] as core::ops::index::Index<core::ops::range::RangeTo<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:391:9
   7: <ash::vk::definitions::PhysicalDeviceMemoryProperties>::memory_heaps_as_slice
             at /home/rafal/.cargo/git/checkouts/ash-b4604afb699e89f5/f4c2ca3/ash/src/vk/definitions.rs:1616:27
   8: repro_ash_src_vk_definitions_rs_1616_27_28d754::main
             at ./src/main.rs:12:47
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
