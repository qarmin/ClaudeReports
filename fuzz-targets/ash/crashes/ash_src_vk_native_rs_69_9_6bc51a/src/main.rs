use ash::vk::native::StdVideoAV1QuantizationFlags;

fn main() {
    let mut bindgen_bitfield_unit = StdVideoAV1QuantizationFlags::new_bitfield_1(0u32, 1u32, 0u32);
    bindgen_bitfield_unit.set(1000000000usize, 0u8, 0u64);
}
