use ash::vk::native::StdVideoH265VpsFlags;

fn main() {
    let mut bindgen_bitfield_unit = StdVideoH265VpsFlags::new_bitfield_1(1u32, 0u32, 0u32, 0u32);
    bindgen_bitfield_unit.set(0usize, 24u8, 0u64);
}
