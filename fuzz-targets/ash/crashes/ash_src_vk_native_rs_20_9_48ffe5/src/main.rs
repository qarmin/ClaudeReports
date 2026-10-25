use ash::vk::native::StdVideoH264SpsVuiFlags;

fn main() {
    let bindgen_bitfield_unit = StdVideoH264SpsVuiFlags::new_bitfield_1(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let _ = bindgen_bitfield_unit.get(1usize, 16u8);
}
