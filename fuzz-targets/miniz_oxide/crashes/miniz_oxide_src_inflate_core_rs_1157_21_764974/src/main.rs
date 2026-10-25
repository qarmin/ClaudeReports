use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let mut decompressor_oxide = DecompressorOxide::new();
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![82u8, 73u8, 70u8, 70u8], &mut vec![0u8, 0u8, 0u8, 0u8, 60u8, 0u8, 86u8, 123u8, 71u8, 66u8, 0u8, 0u8, 0u8, 33u8], 3usize, 10000000000000000000usize, 2388820830u32);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &include_bytes!("image_0.bmp").to_vec(), &mut include_bytes!("image_1.jpg").to_vec(), 2usize, 968484430u32);
}
