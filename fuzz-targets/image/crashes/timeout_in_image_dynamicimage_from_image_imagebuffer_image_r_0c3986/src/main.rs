use image::DynamicImage;
use image::codecs::jpeg::PixelDensity;
use image::codecs::pnm::SampleEncoding;
use image::imageops::BiLevel;
use image::io::DecodedImageAttributes;
use image::metadata::CicpColorPrimaries;

fn main() {
    let cicp_color_primaries = CicpColorPrimaries::RgbB;
    let _ = cicp_color_primaries.is_supported_for_transform();
    let _ = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(include_bytes!("image_0.tiff").to_vec());
    let _ = DecodedImageAttributes::default();
    let _ = SampleEncoding::Binary;
    let _ = PixelDensity::dpi(2048u16);
    let _ = BiLevel;
    let _ = DynamicImage::from(DynamicImage::new_rgba16(2u32, 65536u32).fast_blur(f32::MAX).to_rgba8());
}
