use image::DynamicImage;
use image::codecs::hdr::SIGNATURE;
use image::codecs::pnm::PnmSubtype;
use image::codecs::pnm::SampleEncoding;

fn main() {
    let mut tga_encoder = <image::codecs::tga::TgaEncoder<Vec<u8>>>::new(SIGNATURE.to_vec());
    let _ = <image::codecs::tga::TgaEncoder<Vec<u8>> as image::ImageEncoder>::set_icc_profile(&mut tga_encoder, vec![]);
    let _ = PnmSubtype::Graymap(SampleEncoding::Ascii);
    let dynamic_image = DynamicImage::new_rgb32f(65535u32, 24u32);
    let dynamic_image2 = dynamic_image.thumbnail_exact(26u32, 65535u32);
    let dynamic_image3 = dynamic_image2.thumbnail(65535u32, 16u32);
    let _ = dynamic_image3.huerotate(-1329250857i32);
}
