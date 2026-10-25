use ash::vk::native::StdVideoDecodeH264PictureInfoFlags;

fn main() {
    let bindgen_bitfield_unit = StdVideoDecodeH264PictureInfoFlags::new_bitfield_1(0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let _ = bindgen_bitfield_unit.get(0usize, 100u8);
}
