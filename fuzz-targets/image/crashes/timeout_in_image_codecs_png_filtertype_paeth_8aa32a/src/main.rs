use image::DynamicImage;
use image::Limits;
use image::codecs::bmp::RowsDecoded;
use image::codecs::png::FilterType;
use image::error::ImageFormatHint;
use image::error::UnsupportedErrorKind;
use image::imageops::GaussianBlurParameters;

fn main() {
    let gaussian_blur_parameters = GaussianBlurParameters::SMOOTHING_3;
    let mut limits = Limits::default();
    limits.free_usize(256usize);
    let mut luma = <image::Luma<u8> as ::core::convert::From<[u8; 1]>>::from([254u8]);
    <image::Luma<u8> as image::Pixel>::apply2::<fn(u8, u8) -> u8>(&mut luma, &<image::Luma<u8> as image::Pixel>::to_luma(&<image::Luma<u8> as ::core::convert::From<[u8; 1]>>::from([7u8])), |_, _| 1 as u8);
    let _ = UnsupportedErrorKind::Format(ImageFormatHint::Unknown);
    let dynamic_image = DynamicImage::from(DynamicImage::from(DynamicImage::new_luma_a8(1000u32, 65535u32).to_luma_alpha32f()));
    let dynamic_image2 = dynamic_image.blur_advanced(gaussian_blur_parameters);
    let dynamic_image3 = dynamic_image2.rotate270();
    let _ = dynamic_image3.as_rgb32f();
    let image_buffer = dynamic_image2.as_rgba16().unwrap();
    let static__vec = image_buffer.as_raw();
    let _ = <image::Rgba<u16> as image::Pixel>::from_slice(static__vec);
    let _ = RowsDecoded::BottomUp { rows: 4294967294u32 };
    let image_buffer2 = dynamic_image2.to_rgba32f();
    let static__vec2 = image_buffer2.as_raw();
    let _ = <image::Rgba<f32> as image::Pixel>::from_slice(static__vec2);
    let _ = FilterType::Paeth;
}
