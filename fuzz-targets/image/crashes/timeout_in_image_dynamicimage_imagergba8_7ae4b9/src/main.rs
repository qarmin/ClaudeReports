use image::DynamicImage;
use image::SpecCompliance;
use image::error::LimitErrorKind;
use image::flat::NormalForm;

fn main() {
    let _ = SpecCompliance::Strict;
    let dynamic_image = DynamicImage::ImageRgba8(DynamicImage::new_rgba8(90u32, 62u32).fast_blur(f32::MIN).into_luma8().expand_palette(&[(202u8, 2u8, 154u8), (232u8, 2u8, 27u8), (16u8, 13u8, 8u8), (149u8, 14u8, 69u8), (0u8, 8u8, 87u8), (16u8, 100u8, 100u8)], Some(255u8)));
    let image_buffer = dynamic_image.clone().into_luma16();
    let static__vec = image_buffer.as_raw();
    let _ = <image::Rgba<u16> as image::Pixel>::from_slice(static__vec);
    let _ = LimitErrorKind::from(LimitErrorKind::InsufficientMemory);
    let _ = NormalForm::Unaliased;
}
