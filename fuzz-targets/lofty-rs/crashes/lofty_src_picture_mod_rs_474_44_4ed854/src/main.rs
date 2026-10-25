use lofty::picture::PictureInformation;

fn main() {
    let _ = PictureInformation::from_jpeg(&vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8, 255u8, 192u8, 0u8, 17u8, 8u8, 36u8, 46u8, 39u8, 32u8, 34u8]);
}
