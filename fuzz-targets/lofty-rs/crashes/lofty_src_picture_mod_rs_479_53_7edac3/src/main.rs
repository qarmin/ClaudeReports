use lofty::picture::PictureInformation;

fn main() {
    let _ = PictureInformation::from_jpeg(&vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8]);
}
