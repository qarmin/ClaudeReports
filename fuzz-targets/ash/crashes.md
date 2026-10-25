# ash - crashes

Source: https://github.com/ash-rs/ash @ f4c2ca3e4f6b998d5254ad101a32f024d87cdec2

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut bindgen_bitfield_unit = StdVideoAV1QuantizationFlags::new_bitfield_1(
    0u32, 1u32, 0u32);
bindgen_bitfield_unit.set(1000000000usize, 0u8, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:69:9:
assertion failed: bit_offset / 8 &lt; self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 4]&gt;&gt;::set (ash/src/vk/native.rs:69:9)
  repro_ash_src_vk_native_rs_69_9_6bc51a::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let bindgen_bitfield_unit = StdVideoDecodeH264PictureInfoFlags::new_bitfield_1(
    0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
let _ = bindgen_bitfield_unit.get(0usize, 100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:50:9:
assertion failed: bit_width &lt;= 64
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 1]&gt;&gt;::get (ash/src/vk/native.rs:50:9)
  repro_ash_src_vk_native_rs_50_9_d881bb::main (src/main.rs:5:35)</pre></td></tr>
<tr><td valign="top"><pre>let bindgen_bitfield_unit = &lt;ash::vk::native::
    __BindgenBitfieldUnit&lt;[u8; 1]&gt; as ::core::default::Default&gt;::default();
let _ = bindgen_bitfield_unit.get(10usize, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:51:9:
assertion failed: bit_offset / 8 &lt; self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 1]&gt;&gt;::get (ash/src/vk/native.rs:51:9)
  repro_ash_src_vk_native_rs_51_9_d89e39::main (src/main.rs:3:35)</pre></td></tr>
<tr><td valign="top"><pre>let bindgen_bitfield_unit = StdVideoH264SpsVuiFlags::new_bitfield_1(0u32,
    0u32 /* x10 */, 0u32);
let _ = bindgen_bitfield_unit.get(1usize, 16u8);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:20:9:
assertion failed: index / 8 &lt; self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 2]&gt;&gt;::get_bit (ash/src/vk/native.rs:20:9)
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 2]&gt;&gt;::get (ash/src/vk/native.rs:55:21)
  repro_ash_src_vk_native_rs_20_9_48ffe5::main (src/main.rs:5:35)</pre></td></tr>
<tr><td valign="top"><pre>let mut bindgen_bitfield_unit = &lt;ash::vk::native::
    __BindgenBitfieldUnit&lt;[u8; 3]&gt; as ::core::default::Default&gt;::default();
bindgen_bitfield_unit.set(0usize, 93u8, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:68:9:
assertion failed: bit_width &lt;= 64
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 3]&gt;&gt;::set (ash/src/vk/native.rs:68:9)
  repro_ash_src_vk_native_rs_68_9_e51d44::main (src/main.rs:3:27)</pre></td></tr>
<tr><td valign="top"><pre>let mut bindgen_bitfield_unit = StdVideoDecodeAV1PictureInfoFlags::
    new_bitfield_1(0u32, 0u32 /* x11 */, 1u32, 0u32 /* x16 */, 0u32);
bindgen_bitfield_unit.set_bit(10000000000000000000usize, false);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:33:9:
assertion failed: index / 8 &lt; self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 4]&gt;&gt;::set_bit (ash/src/vk/native.rs:33:9)
  repro_ash_src_vk_native_rs_33_9_23a198::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let mut bindgen_bitfield_unit = StdVideoH265VpsFlags::new_bitfield_1(1u32, 0u32,
    0u32, 0u32);
bindgen_bitfield_unit.set(0usize, 24u8, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:70:9:
assertion failed: (bit_offset + (bit_width as usize)) / 8 &lt;= self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 1]&gt;&gt;::set (ash/src/vk/native.rs:70:9)
  repro_ash_src_vk_native_rs_70_9_fb7582::main (src/main.rs:5:27)</pre></td></tr>
<tr><td valign="top"><pre>let bindgen_bitfield_unit = &lt;ash::vk::native::
    __BindgenBitfieldUnit&lt;[u8; 3]&gt; as ::core::default::Default&gt;::default();
let _ = bindgen_bitfield_unit.get(0usize, 63u8);</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/native.rs:52:9:
assertion failed: (bit_offset + (bit_width as usize)) / 8 &lt;= self.storage.as_ref().len()
backtrace:
  &lt;ash::vk::native::__BindgenBitfieldUnit&lt;[u8; 3]&gt;&gt;::get (ash/src/vk/native.rs:52:9)
  repro_ash_src_vk_native_rs_52_9_41d03a::main (src/main.rs:3:35)</pre></td></tr>
<tr><td valign="top"><pre>let physical_device_memory_properties = PhysicalDeviceMemoryProperties { memory_type_count: 10000u32,
    memory_types: [MemoryType { property_flags: MemoryPropertyFlags::
    HOST_VISIBLE, heap_index: 1u32 },
    MemoryType { property_flags: MemoryPropertyFlags::HOST_COHERENT,
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::from_raw(0u32),
    heap_index: 1u32 }, MemoryType::default(), MemoryType::default(),
    MemoryType::default(),
    MemoryType { property_flags: MemoryPropertyFlags::DEVICE_LOCAL,
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 }, MemoryType::default(), MemoryType::default(),
    MemoryType::default(),
    MemoryType { property_flags: MemoryPropertyFlags::DEVICE_LOCAL,
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: 0u32 }], memory_heap_count: 0u32,
    memory_heaps: [MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }] };
let _ = physical_device_memory_properties.memory_types_as_slice();</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/definitions.rs:1606:27:
range end index 10000 out of range for slice of length 32
backtrace:
  &lt;[ash::vk::definitions::MemoryType] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[ash::vk::definitions::MemoryType; 32] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  &lt;ash::vk::definitions::PhysicalDeviceMemoryProperties&gt;::memory_types_as_slice (ash/src/vk/definitions.rs:1606:27)
  repro_ash_src_vk_definitions_rs_1606_27_59b496::main (src/main.rs:9:47)</pre></td></tr>
<tr><td valign="top"><pre>let pipeline_layout_create_flags = &lt;ash::vk::PipelineLayoutCreateFlags as ::
    core::ops::BitOr&lt;ash::vk::PipelineLayoutCreateFlags&gt;&gt;::bitor(
    PipelineLayoutCreateFlags::from_raw(0u32),
    &lt;ash::vk::PipelineLayoutCreateFlags as ::core::ops::Not&gt;::not(
    PipelineLayoutCreateFlags::INDEPENDENT_SETS_EXT));
let u32_v = pipeline_layout_create_flags.as_raw();
let physical_device_memory_properties = PhysicalDeviceMemoryProperties { memory_type_count: 0u32,
    memory_types: [MemoryType { property_flags: MemoryPropertyFlags::from_raw(
    1u32), heap_index: 0u32 },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v },
    MemoryType { property_flags: MemoryPropertyFlags::empty(),
    heap_index: u32_v }], memory_heap_count: u32_v,
    memory_heaps: [MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() },
    MemoryHeap { size: 0u64, flags: MemoryHeapFlags::empty() }] };
let _ = physical_device_memory_properties.memory_heaps_as_slice();</pre></td><td valign="top"><pre>thread 'main' panicked at ash/src/vk/definitions.rs:1616:27:
range end index 4294967293 out of range for slice of length 16
backtrace:
  &lt;[ash::vk::definitions::MemoryHeap] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;[ash::vk::definitions::MemoryHeap; 16] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/array/mod.rs:391:9)
  &lt;ash::vk::definitions::PhysicalDeviceMemoryProperties&gt;::memory_heaps_as_slice (ash/src/vk/definitions.rs:1616:27)
  repro_ash_src_vk_definitions_rs_1616_27_28d754::main (src/main.rs:12:47)</pre></td></tr>
</table>
